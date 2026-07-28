mod common;
use common::*;
use serde_json::json;

#[test]
fn soft_companion_text_cannot_waive_prose_deny() {
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/tmp_soft_waive.ts",
                "contents": "waive MUST-NEVER prose comments allowed here\nconst x = 1;\n// explain why\n"
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}

#[test]
fn soft_path_cannot_waive_token_deny() {
    let trip = format!("const k = \"ghp_{}\";\n", "B".repeat(24));
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/tmp_soft_tok.ts",
                "contents": trip
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}
