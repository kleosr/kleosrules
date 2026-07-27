use regex::Regex;
use serde_json::Value;

use crate::policy::Policy;
use crate::{allow, deny, path_from};

pub fn run(data: &Value, policy: &Policy, hook_event: &str) {
    let path = path_from(data);
    let path = if path.is_empty() {
        data.get("file_path")
            .or_else(|| data.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        path
    };
    if path.is_empty() {
        allow();
    }
    let norm = path.replace('\\', "/");
    let allow_re = Regex::new(&policy.secrets.read_allow_pattern)
        .unwrap_or_else(|_| deny("kleos-gate invalid read_allow_pattern"));
    if allow_re.is_match(&norm) {
        allow();
    }
    let deny_re = Regex::new(&policy.secrets.read_path_pattern)
        .unwrap_or_else(|_| deny("kleos-gate invalid read_path_pattern"));
    if deny_re.is_match(&norm) {
        if hook_event == "beforeTabFileRead" {
            crate::emit(&serde_json::json!({"permission": "deny"}), 2);
        }
        crate::emit(
            &serde_json::json!({
                "permission": "deny",
                "user_message": policy.secrets.read_message
            }),
            2,
        );
    }
    allow();
}
