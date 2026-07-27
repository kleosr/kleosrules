use serde_json::Value;

use crate::{allow, deny, tool_input, walk_strings};

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
            Some(serde_json::Value::String(s)) => paths.push(s.clone()),
            Some(serde_json::Value::Array(a)) => {
                for x in a {
                    if let Some(s) = x.as_str() {
                        paths.push(s.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    let mut blobs = Vec::new();
    walk_strings(inp, &mut blobs);
    paths.extend(blobs);
    let joined = paths.join(" ");
    if paths.len() > 1 {
        return true;
    }
    for p in &paths {
        let s = p.trim_end_matches('/');
        let base = s.rsplit('/').next().unwrap_or(s);
        if !base.is_empty() && !base.contains('.') {
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
        }
    }
    joined.contains('*') || matches!(joined.trim(), "." | "/" | "~")
}
