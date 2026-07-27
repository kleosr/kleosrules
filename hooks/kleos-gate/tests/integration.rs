use serde_json::{json, Value};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;

fn hooks_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("hooks parent")
        .to_path_buf()
}

fn policy_dir() -> PathBuf {
    hooks_root().join("policy")
}

fn bin_path() -> PathBuf {
    env::var_os("CARGO_BIN_EXE_kleos-gate")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            hooks_root()
                .join("kleos-gate")
                .join("target")
                .join("debug")
                .join("kleos-gate")
        })
}

fn run_gate(event: &str, payload: Value) -> (i32, Value) {
    run_gate_env(event, payload, &policy_dir(), None)
}

fn run_gate_env(event: &str, payload: Value, policy: &Path, state: Option<&Path>) -> (i32, Value) {
    let mut cmd = Command::new(bin_path());
    cmd.arg(event)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy);
    if let Some(st) = state {
        cmd.env("KLEOS_STATE_DIR", st);
    } else {
        let tmp = tempfile_dir();
        cmd.env("KLEOS_STATE_DIR", &tmp);
    }
    let mut child = cmd.spawn().expect("spawn kleos-gate");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin
            .write_all(payload.to_string().as_bytes())
            .expect("write stdin");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let raw = String::from_utf8_lossy(&out.stdout);
    let obj: Value = serde_json::from_str(raw.trim()).unwrap_or(json!({}));
    (code, obj)
}

fn tempfile_dir() -> PathBuf {
    let p = env::temp_dir().join(format!(
        "kleos_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let ignored = std::fs::create_dir_all(&p);
    drop(ignored);
    p
}

fn perm(obj: &Value) -> &str {
    obj.get("permission").and_then(|v| v.as_str()).unwrap_or("")
}

#[test]
fn policy_missing_denies() {
    let empty = tempfile_dir().join("empty_pol");
    let ignored = std::fs::create_dir_all(&empty);
    drop(ignored);
    let (code, obj) = run_gate_env(
        "shell",
        json!({"command": "npm ci"}),
        &empty,
        None,
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn shell_npm_ci_asks() {
    let (code, obj) = run_gate("shell", json!({"command": "npm ci"}));
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "ask");
}

#[test]
fn shell_force_push_denies() {
    let (code, obj) = run_gate(
        "shell",
        json!({"command": "git push origin main --force"}),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn shell_echo_allows() {
    let (code, obj) = run_gate("shell", json!({"command": "echo hi"}));
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn write_prose_denies() {
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "src/a.ts",
                "contents": "const x = 1;\n// explain why\n"
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn write_clean_allows() {
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "src/a.ts",
                "contents": "const x = 1;\n"
            }
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn read_env_denies() {
    let (code, obj) = run_gate(
        "beforeReadFile",
        json!({"hook_event_name": "beforeReadFile", "path": ".env"}),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn mcp_drop_asks() {
    let (code, obj) = run_gate(
        "mcp",
        json!({
            "tool_name": "postgres_drop_table",
            "tool_input": {"table": "users"}
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "ask");
}

#[test]
fn delete_tree_denies() {
    let (code, obj) = run_gate(
        "delete",
        json!({
            "tool_name": "Delete",
            "tool_input": {"path": "payments", "recursive": true}
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn subagent_force_push_denies() {
    let (code, obj) = run_gate(
        "subagentStart",
        json!({
            "hook_event_name": "subagentStart",
            "task": "git push origin main --force"
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn subagent_benign_allows() {
    let (code, obj) = run_gate(
        "subagentStart",
        json!({
            "hook_event_name": "subagentStart",
            "task": "summarize README"
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn prompt_secret_continues_false() {
    let trip = format!("ship ghp_{}", "A".repeat(24));
    let (code, obj) = run_gate(
        "beforeSubmitPrompt",
        json!({
            "hook_event_name": "beforeSubmitPrompt",
            "prompt": trip,
            "attachments": []
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(obj.get("continue").and_then(|v| v.as_bool()), Some(false));
}

#[test]
fn ask_scope_drive_by_asks() {
    let state = tempfile_dir();
    let ignored = run_gate_env(
        "beforeSubmitPrompt",
        json!({
            "hook_event_name": "beforeSubmitPrompt",
            "prompt": "rename foo in auth.ts please",
            "attachments": []
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "utils.ts",
                "contents": "export const n = 1;\n"
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "ask", "drive-by outside prompt paths should ask: {obj}");
}

#[test]
fn hooks_json_no_python3() {
    let hj = hooks_root().join("hooks.json");
    let text = std::fs::read_to_string(&hj).expect("hooks.json");
    assert!(!text.contains("python3"), "hooks.json still references python3");
    assert!(text.contains("kleos-gate"), "hooks.json missing kleos-gate");
    let cfg: Value = serde_json::from_str(&text).expect("json");
    let hooks = cfg.get("hooks").expect("hooks");
    for (event, entries) in hooks.as_object().expect("obj") {
        for entry in entries.as_array().expect("arr") {
            let cmd = entry.get("command").and_then(|v| v.as_str()).unwrap_or("");
            assert!(
                cmd.contains("kleos-gate"),
                "{event} not on kleos-gate: {cmd}"
            );
        }
    }
}

#[test]
fn project_hooks_json_no_python3() {
    let hj = hooks_root().join("hooks.project.json");
    let text = std::fs::read_to_string(&hj).expect("hooks.project.json");
    assert!(!text.contains("python3"));
    assert!(text.contains("kleos-gate"));
}

#[test]
fn write_inline_prose_denies() {
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "src/a.ts",
                "contents": "const x = 1; // why this\n"
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn session_stop_followup_on_unverified() {
    let state = tempfile_dir();
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "t1",
            "tool_name": "Write",
            "tool_input": {"path": "src/a.ts", "contents": "export const n = 1;\n"}
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "stop",
        json!({
            "conversation_id": "t1",
            "status": "completed"
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    let follow = obj.get("followup_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        follow.contains("unverified") || follow.contains("Unverified"),
        "expected stop followup, got {obj}"
    );
}

#[test]
fn write_repeat_deny_escalates() {
    let state = tempfile_dir();
    let payload = json!({
        "conversation_id": "rep1",
        "tool_name": "Write",
        "tool_input": {
            "path": "src/a.ts",
            "contents": "const x = 1;\n// explain why\n"
        }
    });
    let (c1, o1) = run_gate_env("write", payload.clone(), &policy_dir(), Some(&state));
    assert_eq!(c1, 2, "{o1}");
    let (c2, o2) = run_gate_env("write", payload, &policy_dir(), Some(&state));
    assert_eq!(c2, 2, "{o2}");
    let msg = o2.get("agent_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        msg.contains("repeat") || msg.contains("fingerprint"),
        "expected repeat deny message, got {o2}"
    );
}

fn run_check_content(body: &str, path: Option<&str>) -> (i32, String) {
    let mut cmd = Command::new(bin_path());
    cmd.arg("--check-content")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir());
    if let Some(p) = path {
        cmd.arg("--path").arg(p);
    }
    let mut child = cmd.spawn().expect("spawn check-content");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin.write_all(body.as_bytes()).expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    (out.status.code().unwrap_or(1), err)
}

#[test]
fn check_content_prose_denies() {
    let (code, err) = run_check_content("const x = 1;\n// why\n", None);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("prose") || err.contains("NO COMMENTS"), "{err}");
}

#[test]
fn check_content_clean_allows() {
    let (code, err) = run_check_content("export const n = 1;\n", None);
    assert_eq!(code, 0, "{err}");
}

#[test]
fn check_content_path_vernacular_denies() {
    let (code, err) = run_check_content(
        "pub struct X {}\n",
        Some("src/FooUseCase.rs"),
    );
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("vernacular") || err.contains("file-name") || err.contains("path"), "{err}");
}

fn run_cli(args: &[&str]) -> (i32, String, String) {
    let mut cmd = Command::new(bin_path());
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir());
    let out = cmd.output().expect("cli");
    (
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn cli_gate_diff_passes() {
    let (code, stdout, stderr) = run_cli(&["gate-diff"]);
    assert_eq!(code, 0, "stdout={stdout} stderr={stderr}");
    assert!(stdout.contains("GATE_DIFF_PASS"), "{stdout}");
}

#[test]
fn cli_check_user_rules_runs() {
    let (code, _stdout, stderr) = run_cli(&["check-user-rules"]);
    assert!(
        code == 0 || code == 1,
        "unexpected code={code} stderr={stderr}"
    );
}
