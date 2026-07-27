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

fn copy_policy_dir() -> PathBuf {
    let dest = tempfile_dir().join("policy");
    let ignored = std::fs::create_dir_all(&dest);
    drop(ignored);
    for name in [
        "shell.json",
        "lean.json",
        "ask-scope.json",
        "secrets.json",
        "context.json",
    ] {
        std::fs::copy(policy_dir().join(name), dest.join(name)).expect(name);
    }
    dest
}

fn seed_recall(policy: &Path, state: &Path, cid: &str) {
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": cid,
            "tool_name": "CallMcpTool",
            "tool_input": {
                "server": "user-obsidian",
                "toolName": "vault_read",
                "arguments": {"path": "wiki/hot.md"}
            }
        }),
        policy,
        Some(state),
    );
    drop(ignored);
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
fn shell_npm_ci_allows() {
    let (code, obj) = run_gate("shell", json!({"command": "npm ci"}));
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn shell_rm_rf_path_denies() {
    let (code, obj) = run_gate("shell", json!({"command": "rm -rf ./foo"}));
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
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
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "wc1");
    let (code, obj) = run_gate_env(
        "write",
        json!({
            "conversation_id": "wc1",
            "tool_name": "Write",
            "tool_input": {
                "path": "src/a.ts",
                "contents": "const x = 1;\n"
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn write_without_recall_denies() {
    let (code, obj) = run_gate(
        "write",
        json!({
            "conversation_id": "nr1",
            "tool_name": "Write",
            "tool_input": {
                "path": "src/a.ts",
                "contents": "const x = 1;\n"
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
    let msg = obj.get("agent_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        msg.contains("Obsidian recall") || msg.contains("vault_read"),
        "expected recall deny, got {obj}"
    );
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
fn mcp_drop_allows() {
    let (code, obj) = run_gate(
        "mcp",
        json!({
            "tool_name": "postgres_drop_table",
            "tool_input": {"table": "users"}
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
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
fn ask_scope_disabled_allows_drive_by() {
    let pol = copy_policy_dir();
    let raw = std::fs::read_to_string(pol.join("ask-scope.json")).unwrap();
    let mut v: Value = serde_json::from_str(&raw).unwrap();
    v["enabled"] = json!(false);
    std::fs::write(pol.join("ask-scope.json"), v.to_string()).unwrap();
    let state = tempfile_dir();
    seed_recall(&pol, &state, "as1");
    let ignored = run_gate_env(
        "beforeSubmitPrompt",
        json!({
            "conversation_id": "as1",
            "hook_event_name": "beforeSubmitPrompt",
            "prompt": "rename foo in auth.ts please",
            "attachments": []
        }),
        &pol,
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "write",
        json!({
            "conversation_id": "as1",
            "tool_name": "Write",
            "tool_input": {
                "path": "utils.ts",
                "contents": "export const n = 1;\n"
            }
        }),
        &pol,
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}

#[test]
fn ask_scope_enabled_asks_drive_by() {
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "as2");
    let ignored = run_gate_env(
        "beforeSubmitPrompt",
        json!({
            "conversation_id": "as2",
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
            "conversation_id": "as2",
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
    assert_eq!(perm(&obj), "ask");
}

#[test]
fn shell_heredoc_oversize_denies_lean() {
    let mut body = String::new();
    for i in 0..130 {
        body.push_str(&format!("export const x{i} = {i};\n"));
    }
    let cmd = format!("cat > hooks/tmp_shell_big.ts <<'END'\n{body}END\n");
    let (code, obj) = run_gate("shell", json!({ "command": cmd }));
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
    let msg = obj
        .get("agent_message")
        .or_else(|| obj.get("user_message"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(msg.contains("Lean meter"), "{msg}");
}

#[test]
fn shell_tee_code_asks_opaque() {
    let (code, obj) = run_gate(
        "shell",
        json!({ "command": "printf 'export const n = 1\\n' | tee hooks/tmp_tee.ts" }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "ask");
}

#[test]
fn shell_git_apply_asks_opaque() {
    let (code, obj) = run_gate("shell", json!({ "command": "git apply foo.patch" }));
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "ask");
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
    assert!(
        follow.contains("ACT NOW"),
        "expected auto-gauntlet ACT NOW, got {obj}"
    );
    assert!(
        !follow.contains("accept-no-gauntlet-risk"),
        "must not ask human accept-risk, got {obj}"
    );
    assert!(
        follow.contains("Obsidian write-back"),
        "expected Obsidian write-back with unverified, got {obj}"
    );
}

#[test]
fn session_start_injects_obsidian_recall() {
    let (code, obj) = run_gate(
        "sessionStart",
        json!({
            "conversation_id": "s1",
            "hook_event_name": "sessionStart"
        }),
    );
    assert_eq!(code, 0, "{obj}");
    let ctx = obj
        .get("additional_context")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        ctx.contains("Obsidian memory MANDATORY"),
        "expected Obsidian recall inject, got {obj}"
    );
}

#[test]
fn session_start_injects_hot_body() {
    let vault = tempfile_dir();
    let wiki = vault.join("wiki");
    std::fs::create_dir_all(&wiki).unwrap();
    std::fs::write(wiki.join("hot.md"), "# HOT\nkleosr curator fixture marker\n").unwrap();
    std::fs::write(wiki.join("index.md"), "- wiki/projects/kleosr/Index.md\n").unwrap();
    let mut cmd = Command::new(bin_path());
    cmd.arg("sessionStart")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir())
        .env("KLEOS_VAULT", &vault);
    let mut child = cmd.spawn().expect("spawn");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin
            .write_all(
                json!({
                    "conversation_id": "hot1",
                    "hook_event_name": "sessionStart"
                })
                .to_string()
                .as_bytes(),
            )
            .expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let obj: Value = serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim())
        .unwrap_or(json!({}));
    assert_eq!(code, 0, "{obj}");
    let ctx = obj
        .get("additional_context")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        ctx.contains("wiki/hot.md (capped)") && ctx.contains("curator fixture marker"),
        "expected hot body inject, got {obj}"
    );
    assert!(
        ctx.contains("CURATOR") || ctx.contains("context-meter"),
        "expected playbook or meter, got {obj}"
    );
}

#[test]
fn before_prompt_emits_context_pointers() {
    let vault = tempfile_dir();
    let wiki = vault.join("wiki");
    std::fs::create_dir_all(&wiki).unwrap();
    std::fs::write(wiki.join("hot.md"), "# HOT\nkleosr harness pack memory\n").unwrap();
    std::fs::write(
        wiki.join("index.md"),
        "- [[wiki/projects/kleosr/Index]] kleosr harness pack\n- wiki/concepts/memory.md\n",
    )
    .unwrap();
    let mut cmd = Command::new(bin_path());
    cmd.arg("beforeSubmitPrompt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir())
        .env("KLEOS_VAULT", &vault);
    let mut child = cmd.spawn().expect("spawn");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin
            .write_all(
                json!({
                    "hook_event_name": "beforeSubmitPrompt",
                    "prompt": "update the kleosr harness pack memory notes",
                    "attachments": []
                })
                .to_string()
                .as_bytes(),
            )
            .expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let obj: Value = serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim())
        .unwrap_or(json!({}));
    assert_eq!(code, 0, "{obj}");
    let ctx = obj
        .get("additional_context")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        ctx.contains("Obsidian context pointers") || ctx.contains("classify["),
        "expected pointers or classify, got {obj}"
    );
}

#[test]
fn before_prompt_emits_classify_hint() {
    let (code, obj) = run_gate(
        "beforeSubmitPrompt",
        json!({
            "hook_event_name": "beforeSubmitPrompt",
            "prompt": "please fix the rust compile error in the gate",
            "attachments": []
        }),
    );
    assert_eq!(code, 0, "{obj}");
    let ctx = obj
        .get("additional_context")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        ctx.contains("classify[code]"),
        "expected code classify, got {obj}"
    );
}

#[test]
fn session_stop_obsidian_flush_without_vault_write() {
    let state = tempfile_dir();
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "obs1",
            "tool_name": "Shell",
            "tool_input": {"command": "echo hi"}
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "stop",
        json!({
            "conversation_id": "obs1",
            "status": "completed"
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    let follow = obj.get("followup_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        follow.contains("Obsidian write-back"),
        "expected Obsidian write-back, got {obj}"
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
    let root = tempfile_dir();
    std::fs::create_dir_all(root.join("project-rules")).unwrap();
    std::fs::create_dir_all(root.join("hooks/kleos-gate")).unwrap();
    std::fs::write(
        root.join("project-rules/vernacular.mdc"),
        "file_name_pattern: pack_native\nallowed_path_prefixes: hooks/\n",
    )
    .unwrap();
    let mut cmd = Command::new(bin_path());
    cmd.arg("--check-content")
        .arg("--path")
        .arg("src/FooUseCase.rs")
        .current_dir(root.join("hooks/kleos-gate"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir());
    let mut child = cmd.spawn().expect("spawn check-content");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin.write_all(b"pub struct X {}\n").expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(code, 2, "{err}");
    assert!(
        err.contains("vernacular") || err.contains("file-name") || err.contains("path"),
        "{err}"
    );
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

#[test]
fn session_stop_stub_capture_followup() {
    let state = tempfile_dir();
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "stub1",
            "tool_name": "CallMcpTool",
            "tool_input": {
                "server": "user-obsidian",
                "toolName": "vault_append",
                "arguments": {
                    "path": "wiki/projects/kleosr/Sessions/2026-07-27-thin.md",
                    "content": "## Goal\nshort\n"
                }
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "stop",
        json!({
            "conversation_id": "stub1",
            "status": "completed"
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    let follow = obj.get("followup_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        follow.contains("COMPLETE CAPTURE") || follow.contains("thin/stub"),
        "expected stub COMPLETE followup, got {obj}"
    );
    assert!(
        follow.contains("INTENT required") || follow.contains("done-when"),
        "expected INTENT followup with stub session, got {obj}"
    );
}

#[test]
fn session_stop_complete_session_clears_intent_layer() {
    let state = tempfile_dir();
    let body = r#"## Goal
Ship five-layer E2E loop for COMPLETE CAPTURE.

## Done-when
cargo test green and stop followups wired for intent and layer check.

## What ran
Edited session.rs ledger.rs capture.rs and context.json policy.

## Evidence
cargo test -p kleos-gate

## Outcomes
Ledger flags obsidian_complete intent_stated layer_check; stop followups.

## Open
None for this chunk.

## Residual
Stub meter is syntactic only — not semantic completeness.

## Layer check
| Layer | Evidence |
|-------|----------|
| Prompt | Intent + done-when restated |
| Context | vault persist complete |
| Harness | cargo test |
| Loop | This Session |
| Graph | Decisions wikilink planned |
"#;
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "ok1",
            "tool_name": "CallMcpTool",
            "tool_input": {
                "server": "user-obsidian",
                "toolName": "vault_write",
                "arguments": {
                    "path": "wiki/projects/kleosr/Sessions/2026-07-27-ok.md",
                    "content": body
                }
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let (code, obj) = run_gate_env(
        "stop",
        json!({
            "conversation_id": "ok1",
            "status": "completed"
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    let follow = obj.get("followup_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !follow.contains("INTENT required"),
        "complete Session should clear INTENT, got {obj}"
    );
    assert!(
        !follow.contains("LAYER CHECK required"),
        "complete Session with layer table should clear LAYER, got {obj}"
    );
    assert!(
        !follow.contains("COMPLETE CAPTURE"),
        "complete Session should not stub-followup, got {obj}"
    );
}

#[test]
fn before_prompt_does_not_rewrite_user_prompt_field() {
    let (code, obj) = run_gate(
        "beforeSubmitPrompt",
        json!({
            "hook_event_name": "beforeSubmitPrompt",
            "prompt": "UNIQUE_USER_PROMPT_TOKEN_xyz implement the gate",
            "attachments": []
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert!(
        obj.get("prompt").is_none() && obj.get("updated_prompt").is_none(),
        "must not rewrite user prompt fields, got {obj}"
    );
    let ctx = obj
        .get("additional_context")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        ctx.contains("classify[") || ctx.contains("INTENT") || ctx.contains("Ask→"),
        "expected classify/intent inject, got {obj}"
    );
}

#[test]
fn session_stop_layer_check_after_edit() {
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "ly1");
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "ly1",
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/kleos-gate/src/engine/capture.rs",
                "contents": "fn x() {}\n"
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored);
    let ignored2 = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": "ly1",
            "tool_name": "Shell",
            "tool_input": {"command": "cargo test -p kleos-gate"}
        }),
        &policy_dir(),
        Some(&state),
    );
    drop(ignored2);
    let (code, obj) = run_gate_env(
        "stop",
        json!({
            "conversation_id": "ly1",
            "status": "completed"
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 0, "{obj}");
    let follow = obj.get("followup_message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        follow.contains("LAYER CHECK"),
        "edit session without layer_check should followup, got {obj}"
    );
}

