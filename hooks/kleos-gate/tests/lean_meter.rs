mod common;
use common::*;
use serde_json::json;

#[test]
fn write_new_file_over_loc_denies_lean() {
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "ln1");
    let mut body = String::new();
    for i in 0..130 {
        body.push_str(&format!("export const x{i} = {i};\n"));
    }
    let (code, obj) = run_gate_env(
        "write",
        json!({
            "conversation_id": "ln1",
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/tmp_lean_new.ts",
                "contents": body
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
    let msg = obj
        .get("agent_message")
        .or_else(|| obj.get("user_message"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(msg.contains("Lean") || msg.contains("lean"), "{msg}");
}

#[test]
fn write_absolute_file_loc_max_denies() {
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "ln2");
    let dir = tempfile_dir();
    let path = dir.join("abs_roof.ts");
    let mut existing = String::new();
    for i in 0..690 {
        existing.push_str(&format!("export const a{i} = {i};\n"));
    }
    std::fs::write(&path, &existing).unwrap();
    let mut next = String::new();
    for i in 0..720 {
        next.push_str(&format!("export const a{i} = {i};\n"));
    }
    let (code, obj) = run_gate_env(
        "write",
        json!({
            "conversation_id": "ln2",
            "tool_name": "Write",
            "tool_input": {
                "path": path.to_string_lossy(),
                "contents": next
            }
        }),
        &policy_dir(),
        Some(&state),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
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
