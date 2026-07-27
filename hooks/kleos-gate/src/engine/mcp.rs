use regex::Regex;
use serde_json::Value;

use crate::policy::Policy;
use crate::{ask, allow, deny, walk_strings};

pub fn run(data: &Value, policy: &Policy) {
    let tool = data
        .get("tool_name")
        .or_else(|| data.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let mut blobs = Vec::new();
    walk_strings(data, &mut blobs);
    let text = blobs.join("\n");
    if !text.is_empty()
        && !text.contains(&policy.secrets.allow_substring)
    {
        if let Ok(re) = Regex::new(&policy.secrets.content_pattern) {
            if re.is_match(&text) {
                deny(&policy.secrets.mcp_secret_message);
            }
        }
    }
    let joined = format!("{tool}\n{text}");
    let danger = Regex::new(&policy.secrets.mcp_danger_pattern)
        .unwrap_or_else(|_| deny("kleos-gate invalid mcp_danger_pattern"));
    if danger.is_match(&joined) {
        ask(&policy.secrets.mcp_ask_message);
    }
    allow();
}
