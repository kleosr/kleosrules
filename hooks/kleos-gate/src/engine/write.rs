use serde_json::{json, Value};
use std::path::PathBuf;

use crate::policy::Policy;
use crate::{allow, deny, path_from, tool_input, walk_strings};
use super::context;
use super::lean;
use super::ledger;

const REPEAT_MSG: &str =
    "Blocked repeat deny of the same write fingerprint. Rewrite to an allowed surface or ask the user — do not retry the same blocked payload.";

pub fn run(data: &Value, policy: &Policy, state: &PathBuf, _hooks: &PathBuf) {
    let tool = data
        .get("tool_name")
        .or_else(|| data.get("toolName"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let event = data
        .get("hook_event_name")
        .and_then(|v| v.as_str())
        .unwrap_or("preToolUse");
    if event == "preToolUse" && tool == "Delete" {
        crate::engine::delete::run(data);
    }

    let inp = tool_input(data);
    let mut blobs = Vec::new();
    walk_strings(inp, &mut blobs);
    walk_strings(data, &mut blobs);
    let text = blobs.join("\n");
    if let Some(msg) = scan_cred(&text, policy) {
        if event == "beforeSubmitPrompt" {
            crate::emit(
                &serde_json::json!({"continue": false, "user_message": msg}),
                2,
            );
        }
        deny(&msg);
    }

    let path = path_from(data);
    let mut bodies = Vec::new();
    if let Some(s) = inp.get("contents").and_then(|v| v.as_str()) {
        bodies.push(s.to_string());
    }
    if let Some(s) = inp.get("new_string").and_then(|v| v.as_str()) {
        bodies.push(s.to_string());
    }
    if let Some(s) = inp.get("new_source").and_then(|v| v.as_str()) {
        bodies.push(s.to_string());
    }
    if let Some(s) = inp.get("cell_content").and_then(|v| v.as_str()) {
        bodies.push(s.to_string());
    }
    let body = bodies.join("\n");

    if crate::engine::prose::has_prose(&body) {
        deny_with_repeat(data, state, &path, &body, event, "Blocked prose comment in code write (Native Lean NO COMMENTS).");
    }

    if !path.is_empty() && lean::is_code_path(&path, &policy.lean) {
        if policy.context.recall_gate_enabled
            && !context::is_write_exempt(&path, &policy.context)
            && !ledger::freshness(state, &ledger::conversation_id(data)).recall
        {
            deny_with_repeat(
                data,
                state,
                &path,
                &body,
                event,
                &policy.context.recall_message,
            );
        }
        if let Err(reason) = crate::engine::vernacular::check_body_and_path(&path, &body) {
            deny_with_repeat(data, state, &path, &body, event, &reason);
        }
        let contents = inp.get("contents").and_then(|v| v.as_str());
        let old = inp.get("old_string").and_then(|v| v.as_str());
        let new = inp.get("new_string").and_then(|v| v.as_str());
        if let Some(reason) = lean::check(&path, contents, old, new, &policy.lean) {
            deny_with_repeat(data, state, &path, &body, event, &reason);
        }
    }
    if !path.is_empty() {
        crate::engine::ask_scope::check_path(data, policy, state);
    }

    allow();
}

fn deny_with_repeat(
    data: &Value,
    state: &PathBuf,
    path: &str,
    body: &str,
    event: &str,
    msg: &str,
) -> ! {
    let cid = ledger::conversation_id(data);
    let fp = ledger::fingerprint(&json!({"path": path, "body": body, "event": event}));
    let prior = ledger::prior_deny_count(state, &cid, &fp);
    let _ = ledger::record_deny_fp(state, &cid, &fp);
    if prior >= 1 {
        ledger::append_event(
            state,
            &cid,
            "deny_repeat",
            json!({"fp": fp, "path": path}),
        );
        deny(REPEAT_MSG);
    }
    deny(msg);
}

fn scan_cred(text: &str, policy: &Policy) -> Option<String> {
    if text.is_empty() || text.contains(&policy.secrets.allow_substring) {
        return None;
    }
    let re = regex::Regex::new(&policy.secrets.content_pattern).ok()?;
    if re.is_match(text) {
        Some(policy.secrets.content_message.clone())
    } else {
        None
    }
}
