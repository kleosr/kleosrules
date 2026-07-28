mod common;
use common::*;
use serde_json::json;

#[test]
fn write_token_pattern_denies() {
    let trip = format!("const k = \"ghp_{}\";\n", "A".repeat(24));
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/tmp_tok.ts",
                "contents": trip
            }
        }),
    );
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
    let msg = obj
        .get("agent_message")
        .or_else(|| obj.get("user_message"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(msg.to_lowercase().contains("secret"), "{msg}");
}

#[test]
fn write_token_allow_substring_allows() {
    let token = format!("ghp_{}", "A".repeat(24));
    let body = format!("const k = \"EXAMPLE_SECRET_{token}\";\n");
    let (code, obj) = run_gate(
        "write",
        json!({
            "tool_name": "Write",
            "tool_input": {
                "path": "hooks/tmp_tok_allow.ts",
                "contents": body
            }
        }),
    );
    assert_eq!(code, 0, "{obj}");
    assert_eq!(perm(&obj), "allow");
}
