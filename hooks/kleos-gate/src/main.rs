use serde_json::{json, Value};
use std::env;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

mod engine;
mod fleet;
mod policy;

use engine::{ask_scope, context, delete, mcp, read, session, shell, subagent, write};
use policy::Policy;

fn hooks_dir() -> PathBuf {
    if let Ok(p) = env::var("KLEOS_HOOKS_DIR") {
        return PathBuf::from(p);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(bin) = exe.parent() {
            let up = bin.join("..");
            if up.join("policy").is_dir() {
                return up.canonicalize().unwrap_or(up);
            }
            if bin.join("policy").is_dir() {
                return bin.to_path_buf();
            }
        }
    }
    PathBuf::from("hooks")
}

fn policy_dir(hooks: &Path) -> PathBuf {
    if let Ok(p) = env::var("KLEOS_POLICY_DIR") {
        return PathBuf::from(p);
    }
    hooks.join("policy")
}

fn state_dir() -> PathBuf {
    if let Ok(p) = env::var("KLEOS_STATE_DIR") {
        return PathBuf::from(p);
    }
    env::var_os("HOME")
        .map(|h| PathBuf::from(h).join(".cursor").join("hooks-state"))
        .unwrap_or_else(|| PathBuf::from("/tmp/kleos-hooks-state"))
}

fn read_stdin() -> Value {
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        return json!({"_parse_error": true});
    }
    if buf.trim().is_empty() {
        return json!({});
    }
    serde_json::from_str(&buf).unwrap_or_else(|_| json!({"_parse_error": true}))
}

pub(crate) fn emit(v: &Value, code: i32) -> ! {
    let ignored = writeln!(io::stdout(), "{v}");
    drop(ignored);
    let ignored2 = io::stdout().flush();
    drop(ignored2);
    process::exit(code);
}

pub(crate) fn deny(msg: &str) -> ! {
    emit(
        &json!({
            "permission": "deny",
            "user_message": msg,
            "agent_message": msg
        }),
        2,
    );
}

pub(crate) fn ask(msg: &str) -> ! {
    emit(
        &json!({
            "permission": "ask",
            "user_message": msg,
            "agent_message": msg
        }),
        0,
    );
}

pub(crate) fn allow() -> ! {
    emit(&json!({"permission": "allow"}), 0);
}

pub(crate) fn allow_empty_prompt() -> ! {
    emit(&json!({}), 0);
}

fn main() {
    std::panic::set_hook(Box::new(|_info| {}));
    let result = std::panic::catch_unwind(run);
    match result {
        Ok(()) => {}
        Err(payload) => {
            drop(payload);
            deny("kleos-gate failed closed");
        }
    }
}

fn check_fail(msg: &str) -> ! {
    let ignored = writeln!(io::stderr(), "{msg}");
    drop(ignored);
    let ignored2 = io::stderr().flush();
    drop(ignored2);
    process::exit(2);
}

fn check_content(args: &[String]) {
    let mut path = String::new();
    let mut i = 0;
    while i < args.len() {
        if (args[i] == "--path" || args[i] == "-p") && i + 1 < args.len() {
            path = args[i + 1].clone();
            i += 2;
            continue;
        }
        i += 1;
    }
    let mut body = String::new();
    if io::stdin().read_to_string(&mut body).is_err() {
        check_fail("kleos-gate --check-content: stdin read failed");
    }
    let hooks = hooks_dir();
    let pdir = policy_dir(&hooks);
    let policy = match Policy::load(&pdir) {
        Ok(p) => p,
        Err(e) => check_fail(&format!("kleos-gate policy missing/invalid: {e}")),
    };
    if engine::prose::has_prose(&body) {
        check_fail("Blocked prose comment in code write (Native Lean NO COMMENTS).");
    }
    if path.is_empty() {
        if engine::lean::enabled(&policy.lean) {
            let n = body.matches('\n').count()
                + usize::from(!body.is_empty() && !body.ends_with('\n'));
            let lim = env::var(&policy.lean.new_file_loc_env)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(policy.lean.new_file_loc);
            if n > lim {
                check_fail(&format!(
                    "Lean meter: content {n} LOC > {lim} (reuse/split or KLEOS_LEAN=0 / KLEOS_LEAN_NEW_FILE_LOC)"
                ));
            }
        }
        process::exit(0);
    }
    if engine::lean::is_code_path(&path, &policy.lean) {
        if let Err(reason) = engine::vernacular::check_body_and_path(&path, &body) {
            check_fail(&reason);
        }
        if let Some(reason) =
            engine::lean::check(&path, Some(&body), None, None, &policy.lean)
        {
            check_fail(&reason);
        }
    }
    process::exit(0);
}

fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--check-content" || a == "check-content") {
        check_content(&args);
        return;
    }
    let event_arg = args.first().map(|s| s.as_str()).unwrap_or("auto");
    if matches!(
        event_arg,
        "gate-diff"
            | "obedience-report"
            | "check-user-rules"
            | "install"
            | "install-hooks"
            | "sync"
            | "sync-hooks"
            | "verify"
            | "bench"
            | "discover"
            | "install-pre-commit"
    ) {
        let hooks = hooks_dir();
        let pdir = policy_dir(&hooks);
        let policy = match Policy::load(&pdir) {
            Ok(p) => p,
            Err(e) => deny(&format!("kleos-gate policy missing/invalid: {e}")),
        };
        let st = state_dir();
        match event_arg {
            "gate-diff" => engine::tools::gate_diff(&hooks, &policy),
            "obedience-report" => engine::tools::obedience_report(&st),
            "check-user-rules" => engine::tools::check_user_rules(&hooks),
            "install" => fleet::install::run(&hooks),
            "install-hooks" => fleet::install::run_hooks(&hooks),
            "sync" => fleet::sync::run(&hooks),
            "sync-hooks" => fleet::sync::run_hooks(&hooks),
            "verify" => fleet::verify::run(&hooks),
            "bench" => fleet::bench::run(&hooks),
            "discover" => fleet::discover::run(&hooks),
            "install-pre-commit" => fleet::pre_commit::run(&hooks),
            _ => unreachable!(),
        }
    }
    let data = read_stdin();
    if data.get("_parse_error").and_then(|v| v.as_bool()) == Some(true) {
        deny("kleos-gate JSON parse error");
    }

    let hooks = hooks_dir();
    let pdir = policy_dir(&hooks);
    let policy = match Policy::load(&pdir) {
        Ok(p) => p,
        Err(e) => deny(&format!("kleos-gate policy missing/invalid: {e}")),
    };

    let hook_event = data
        .get("hook_event_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let event = if event_arg == "auto" {
        if !hook_event.is_empty() {
            hook_event
        } else {
            "preToolUse"
        }
    } else {
        event_arg
    };

    let st = state_dir();
    let ignored = std::fs::create_dir_all(&st);
    drop(ignored);

    match event {
        "beforeSubmitPrompt" | "prompt" | "block-secrets" => {
            before_prompt(&data, &policy, &st);
        }
        "beforeShellExecution" | "shell" | "gate-shell" => {
            shell::run(&data, &policy);
        }
        "preToolUse" | "write" | "gate-write" => {
            let tool = data
                .get("tool_name")
                .or_else(|| data.get("toolName"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if tool == "Delete" || tool.contains("Delete") {
                delete::run(&data);
            } else {
                write::run(&data, &policy, &st, &hooks);
            }
        }
        "delete" | "gate-delete" => delete::run(&data),
        "beforeReadFile" | "beforeTabFileRead" | "read" | "gate-read" => {
            read::run(&data, &policy, hook_event);
        }
        "beforeMCPExecution" | "mcp" | "gate-mcp" => mcp::run(&data, &policy),
        "postToolUse" | "session-ledger" => session::post_tool_use(&data, &st),
        "postToolUseFailure" | "gate-fail" => allow(),
        "subagentStart" | "gate-subagent" => subagent::run_start(&data),
        "subagentStop" => subagent::run_stop(&data),
        "sessionStart" | "preCompact" | "session-boundary" => {
            session::session_boundary(&data, &st, event, &policy)
        }
        "stop" | "stop-verify" => session::stop_verify(&data, &st),
        other => deny(&format!("kleos-gate unknown event: {other}")),
    }
}

fn before_prompt(data: &Value, policy: &Policy, state: &PathBuf) {
    let mut blobs = Vec::new();
    walk_strings(data, &mut blobs);
    let text = blobs.join("\n");
    if !text.is_empty() && !text.contains(&policy.secrets.allow_substring) {
        if let Ok(re) = regex::Regex::new(&policy.secrets.content_pattern) {
            if re.is_match(&text) {
                emit(
                    &json!({
                        "continue": false,
                        "user_message": policy.secrets.content_message
                    }),
                    2,
                );
            }
        }
    }
    ask_scope::record_prompt(data, policy, state);
    if let Some(ctx) = context::ask_context(data, &policy.context) {
        emit(&json!({ "additional_context": ctx }), 0);
    }
    allow_empty_prompt();
}

pub(crate) fn path_from(data: &Value) -> String {
    let inp = tool_input(data);
    for k in ["path", "file_path", "target_notebook", "filePath"] {
        if let Some(s) = inp.get(k).and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return s.to_string();
            }
        }
        if let Some(s) = data.get(k).and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return s.to_string();
            }
        }
    }
    String::new()
}

pub(crate) fn tool_input(data: &Value) -> &Value {
    data.get("tool_input")
        .or_else(|| data.get("input"))
        .or_else(|| data.get("arguments"))
        .unwrap_or(data)
}

pub(crate) fn command_from(data: &Value) -> String {
    let inp = tool_input(data);
    for k in ["command", "cmd"] {
        if let Some(s) = inp.get(k).and_then(|v| v.as_str()) {
            return s.to_string();
        }
        if let Some(s) = data.get(k).and_then(|v| v.as_str()) {
            return s.to_string();
        }
    }
    String::new()
}

pub(crate) fn walk_strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) if !s.trim().is_empty() => out.push(s.clone()),
        Value::Array(a) => {
            for x in a {
                walk_strings(x, out);
            }
        }
        Value::Object(m) => {
            for x in m.values() {
                walk_strings(x, out);
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
pub(crate) fn content_blobs(v: &Value, out: &mut Vec<String>) {
    const KEYS: &[&str] = &[
        "contents",
        "content",
        "new_string",
        "old_string",
        "cell_content",
        "new_source",
        "command",
        "cmd",
    ];
    match v {
        Value::Object(m) => {
            for (k, val) in m {
                if KEYS.contains(&k.as_str()) {
                    if let Some(s) = val.as_str() {
                        if !s.trim().is_empty() {
                            out.push(s.to_string());
                        }
                    }
                } else {
                    content_blobs(val, out);
                }
            }
        }
        Value::Array(a) => {
            for x in a {
                content_blobs(x, out);
            }
        }
        _ => {}
    }
}
