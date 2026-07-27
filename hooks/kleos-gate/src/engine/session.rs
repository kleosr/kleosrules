use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::OnceLock;

use super::injection;
use super::ledger;
use crate::{allow_empty_prompt, command_from, emit, path_from, walk_strings};

const ROOF: &str = "Master Mind roof: NO prose comments; ASK package installs; no remote publish without confirmation; verify before Done; never fight a deny.";
const ROOF_EVERY: usize = 12;
const FOLLOW: &str = "Session has unverified edits. Run the house gauntlet (TOOLCHAIN / tests) and cite evidence, or name residual risk and ask accept-no-gauntlet-risk. Do not claim Done without verification evidence.";
const LOOP_MSG: &str = "Freeze loop detected (repeat deny fingerprints). Stop retrying the same blocked write; rewrite to an allowed surface or ask the user.";

fn verify_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r"(?i)(^|[;&|]\s*)(npm\s+test|pnpm\s+test|yarn\s+test|bun\s+test|pytest|python\s+-m\s+pytest|cargo\s+test|go\s+test|tsc(\s|$)|ruff\s+check|eslint|gradlew?\s+test|gradlew?\s+check|make\s+test|cmake\s+--build|bash\s+.*TOOLCHAIN|npm\s+run\s+test)",
        )
        .expect("verify regex")
    })
}

fn is_edit_tool(tool: &str) -> bool {
    matches!(
        tool,
        "Write" | "StrReplace" | "EditNotebook" | "EditFile" | "Delete" | "ApplyPatch"
    ) || tool.contains("Write")
        || tool.contains("StrReplace")
        || tool.contains("Edit")
        || tool.contains("Delete")
}

fn tool_result_text(data: &Value) -> String {
    let mut parts = Vec::new();
    for key in ["result", "output", "content", "text", "stdout", "stderr"] {
        if let Some(v) = data.get(key) {
            walk_strings(v, &mut parts);
        }
    }
    if let Some(tr) = data.get("tool_result") {
        walk_strings(tr, &mut parts);
    }
    parts.join("\n")
}

fn mcp_output_text(data: &Value) -> String {
    for key in ["mcp_tool_output", "tool_output", "output"] {
        if let Some(v) = data.get(key) {
            if let Some(s) = v.as_str() {
                return s.to_string();
            }
            let mut parts = Vec::new();
            walk_strings(v, &mut parts);
            if !parts.is_empty() {
                return parts.join("\n");
            }
        }
    }
    String::new()
}

pub fn post_tool_use(data: &Value, state: &Path) {
    let cid = ledger::conversation_id(data);
    let tool = data
        .get("tool_name")
        .or_else(|| data.get("toolName"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    ledger::append_event(state, &cid, "tool", json!({"name": tool}));

    if is_edit_tool(tool) {
        let path = path_from(data);
        if !path.is_empty()
            && !path.ends_with(".md")
            && !path.ends_with(".mdc")
            && !path.ends_with(".txt")
            && !path.ends_with(".json")
        {
            ledger::append_event(state, &cid, "edit", json!({"path": path}));
        }
    }

    let cmd = command_from(data);
    if !cmd.is_empty() && verify_re().is_match(&cmd) {
        ledger::append_event(
            state,
            &cid,
            "verify",
            json!({"cmd": cmd.chars().take(200).collect::<String>()}),
        );
    }

    let mut contexts: Vec<String> = Vec::new();
    let inj_text = tool_result_text(data);
    if tool != "Shell"
        && !inj_text.is_empty()
        && injection::is_injection(&inj_text)
    {
        contexts.push(injection::notice(&inj_text));
    }
    let mcp_text = mcp_output_text(data);
    let mut out = json!({});
    if tool != "Shell"
        && !mcp_text.is_empty()
        && injection::is_injection(&mcp_text)
    {
        let n = injection::notice(&mcp_text);
        contexts.push(n.clone());
        out["updated_mcp_tool_output"] = json!(serde_json::to_string(&json!({
            "warning": n,
            "filtered": true
        }))
        .unwrap_or_default());
    }

    for ev in ledger::read_events(state, &cid).iter().rev().take(8) {
        if ev.get("normalized").and_then(|v| v.as_bool()) == Some(true) {
            contexts.push("Normalize write stripped prose comments from payload.".into());
            break;
        }
    }

    let fresh = ledger::freshness(state, &cid);
    if fresh.tools > 0 && fresh.tools % ROOF_EVERY == 0 {
        contexts.push(ROOF.into());
    }

    if !contexts.is_empty() {
        out["additional_context"] = json!(contexts.join(" "));
    }
    if out.as_object().map(|m| !m.is_empty()).unwrap_or(false) {
        emit(&out, 0);
    }
    emit(&json!({}), 0);
}

pub fn session_boundary(data: &Value, state: &Path, hook_event: &str) {
    let cid = ledger::conversation_id(data);
    if hook_event == "sessionStart" || hook_event == "session-boundary" {
        let fresh = ledger::freshness(state, &cid);
        let mut parts = vec![ROOF.to_string()];
        if !fresh.unverified.is_empty() {
            parts.push(format!(
                "Session carry-over unverified paths: {}. Run verify-class commands before claiming Done.",
                fresh.unverified.join(", ")
            ));
        }
        emit(&json!({"additional_context": parts.join(" ")}), 0);
    }
    if hook_event == "preCompact" {
        ledger::append_event(state, &cid, "compact", json!({}));
        let fresh = ledger::freshness(state, &cid);
        if !fresh.unverified.is_empty() {
            emit(
                &json!({
                    "user_message": format!(
                        "Compaction pending — unverified edits on: {}. Re-verify after resume.",
                        fresh.unverified.join(", ")
                    )
                }),
                0,
            );
        }
        emit(&json!({}), 0);
    }
    allow_empty_prompt();
}

pub fn stop_verify(data: &Value, state: &Path) {
    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
    if !status.is_empty() && status != "completed" {
        allow_empty_prompt();
    }
    let cid = ledger::conversation_id(data);
    let fresh = ledger::freshness(state, &cid);
    if fresh.loops >= 2 {
        emit(
            &json!({
                "followup_message": format!(
                    "{} Unverified: {}",
                    LOOP_MSG,
                    fresh.unverified.join(", ")
                )
            }),
            0,
        );
    }
    if !fresh.unverified.is_empty() {
        emit(
            &json!({
                "followup_message": format!(
                    "{} Unverified paths: {}",
                    FOLLOW,
                    fresh.unverified.join(", ")
                )
            }),
            0,
        );
    }
    allow_empty_prompt();
}

