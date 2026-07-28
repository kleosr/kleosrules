#![allow(dead_code)]

use serde_json::{json, Value};
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn hooks_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("hooks parent")
        .to_path_buf()
}

pub fn policy_dir() -> PathBuf {
    hooks_root().join("policy")
}

pub fn copy_policy_dir() -> PathBuf {
    let dest = tempfile_dir().join("policy");
    let ignored = std::fs::create_dir_all(&dest);
    drop(ignored);
    for name in [
        "shell.json",
        "lean.json",
        "ask-scope.json",
        "secrets.json",
        "context.json",
        "delete.json",
    ] {
        std::fs::copy(policy_dir().join(name), dest.join(name)).expect(name);
    }
    dest
}

pub fn seed_recall(policy: &Path, state: &Path, cid: &str) {
    let ignored = run_gate_env(
        "postToolUse",
        json!({
            "conversation_id": cid,
            "tool_name": "MCP:vault_read",
            "tool_input": {"path": "wiki/hot.md"}
        }),
        policy,
        Some(state),
    );
    drop(ignored);
}

pub fn seed_recall_callmcp(policy: &Path, state: &Path, cid: &str) {
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

pub fn bin_path() -> PathBuf {
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

pub fn run_gate(event: &str, payload: Value) -> (i32, Value) {
    run_gate_env(event, payload, &policy_dir(), None)
}

pub fn run_gate_env(event: &str, payload: Value, policy: &Path, state: Option<&Path>) -> (i32, Value) {
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

pub fn tempfile_dir() -> PathBuf {
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

pub fn perm(obj: &Value) -> &str {
    obj.get("permission").and_then(|v| v.as_str()).unwrap_or("")
}

pub fn run_check_content(body: &str, path: Option<&str>) -> (i32, String) {
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

pub fn run_cli(args: &[&str]) -> (i32, String, String) {
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
