use regex::Regex;
use serde_json::Value;

use crate::{allow, deny, walk_strings};

const DENY_BRIEF: &str = "Blocked subagent brief with gated ASK-ONCE/MUST-NEVER actions or injection frames. Rewrite brief without force-push/publish/destructive wipe commands or override frames.";

pub fn run_start(data: &Value) {
    let text = brief_text(data);
    if brief_denied(&text) {
        deny(DENY_BRIEF);
    }
    allow();
}

pub fn run_stop(_data: &Value) {
    crate::emit(&serde_json::json!({}), 0);
}

fn brief_text(data: &Value) -> String {
    let mut parts = Vec::new();
    for k in ["task", "brief", "prompt", "description", "message"] {
        if let Some(s) = data.get(k).and_then(|v| v.as_str()) {
            if !s.trim().is_empty() {
                parts.push(s.to_string());
            }
        }
    }
    for k in ["subagent", "agent"] {
        if let Some(obj) = data.get(k).and_then(|v| v.as_object()) {
            for kk in ["task", "brief", "prompt", "description"] {
                if let Some(s) = obj.get(kk).and_then(|v| v.as_str()) {
                    if !s.trim().is_empty() {
                        parts.push(s.to_string());
                    }
                }
            }
        }
    }
    let mut blobs = Vec::new();
    walk_strings(data, &mut blobs);
    parts.extend(blobs);
    parts.join("\n")
}

fn brief_denied(text: &str) -> bool {
    if text.trim().is_empty() {
        return false;
    }
    let re = Regex::new(
        r"(?i)(?:git\s+push[^;&|]*(?:--force| -f\s|--force-with-lease)|git\s+reset\s+--hard|git\s+clean\s+-[a-z]*f|rm\s+-[a-z]*rf|npm\s+publish|terraform\s+destroy|curl[^\n]*\|\s*(?:sh|bash)|gh\s+release\s+create|docker\s+push|find\s+.*-delete)",
    )
    .expect("subagent gated re");
    re.is_match(text)
}
