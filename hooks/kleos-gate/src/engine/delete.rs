use serde_json::Value;

use crate::{allow, deny, tool_input};

pub fn run(data: &Value) {
    if is_treeish(data) {
        deny(
            "Blocked tree/mass Delete via native tool. Native Delete cannot ask (preToolUse ask is unenforced). State the exact path list, get user assent, then delete via Shell so beforeShellExecution can ask.",
        );
    }
    allow();
}

fn is_treeish(data: &Value) -> bool {
    let inp = tool_input(data);
    if matches!(
        inp.get("recursive"),
        Some(v) if v.as_bool() == Some(true)
            || v.as_str() == Some("true")
            || v.as_str() == Some("True")
            || v.as_i64() == Some(1)
    ) {
        return true;
    }
    let mut paths: Vec<String> = Vec::new();
    for k in ["path", "paths", "files", "targets", "file_path"] {
        match inp.get(k).or_else(|| data.get(k)) {
            Some(serde_json::Value::String(s)) => {
                if !s.is_empty() && !paths.contains(s) {
                    paths.push(s.clone());
                }
            }
            Some(serde_json::Value::Array(a)) => {
                for x in a {
                    if let Some(s) = x.as_str() {
                        if !s.is_empty() && !paths.iter().any(|p| p == s) {
                            paths.push(s.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if paths.is_empty() {
        return false;
    }
    if paths.len() > 1 {
        return true;
    }
    let s = paths[0].trim_end_matches('/');
    let base = s.rsplit('/').next().unwrap_or(s);
    if base.contains('.') {
        return s.contains('*') || matches!(s.trim(), "." | "/" | "~");
    }
    if s.ends_with("src")
        || s.ends_with("lib")
        || s.ends_with("app")
        || s.ends_with("packages")
        || s.ends_with("payments")
        || s.ends_with("ledger")
    {
        return true;
    }
    if s.contains('/') && !base.contains('.') {
        return true;
    }
    s.contains('*') || matches!(s.trim(), "." | "/" | "~")
}
