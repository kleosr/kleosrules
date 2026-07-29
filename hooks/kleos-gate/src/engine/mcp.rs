use regex::Regex;
use serde_json::Value;
use std::path::Path;

use super::session;
use crate::policy::Policy;
use crate::{allow, ask, deny, walk_strings};

pub fn run(data: &Value, policy: &Policy, state: &Path) {
    let tool = data
        .get("tool_name")
        .or_else(|| data.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let mut blobs = Vec::new();
    walk_strings(data, &mut blobs);
    let text = blobs.join("\n");
    if !text.is_empty() && !text.contains(&policy.secrets.allow_substring) {
        if let Ok(re) = Regex::new(&policy.secrets.content_pattern) {
            if re.is_match(&text) {
                deny(&policy.secrets.mcp_secret_message);
            }
        }
    }
    let joined = format!("{tool}\n{text}");
    let danger_pat = &policy.secrets.mcp_danger_pattern;
    let danger_lower = danger_pat.to_lowercase();
    let is_dead_pattern = danger_lower == "a^" || danger_lower.is_empty();
    let tool_lower = tool.to_lowercase();
    let joined_lower = joined.to_lowercase();
    let is_mcp_write = tool_lower.contains("callmcp")
        || tool_lower.contains("mcp:")
        || joined_lower.contains("vault_write")
        || joined_lower.contains("vault_append")
        || joined_lower.contains("vault_delete");
    if is_dead_pattern && is_mcp_write {
        deny(&policy.secrets.mcp_ask_message);
        return;
    }
    if !is_dead_pattern {
        let danger = Regex::new(danger_pat)
            .unwrap_or_else(|_| deny("kleos-gate invalid mcp_danger_pattern"));
        if danger.is_match(&joined) {
            ask(&policy.secrets.mcp_ask_message);
        }
    }
    session::note_obsidian_events(data, state);
    allow();
}