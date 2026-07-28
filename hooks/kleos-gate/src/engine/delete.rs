use serde_json::Value;

use crate::policy::{DeletePolicy, Policy};
use crate::{allow, deny, tool_input};

pub fn run(data: &Value, policy: &Policy) {
    if is_treeish(data, &policy.delete) {
        deny(&policy.delete.message);
    }
    allow();
}

fn is_treeish(data: &Value, pol: &DeletePolicy) -> bool {
    let inp = tool_input(data);
    if pol.deny_recursive
        && matches!(
            inp.get("recursive"),
            Some(v) if v.as_bool() == Some(true)
                || v.as_str() == Some("true")
                || v.as_str() == Some("True")
                || v.as_i64() == Some(1)
        )
    {
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
    if pol.deny_multi_path && paths.len() > 1 {
        return true;
    }
    let s = paths[0].trim_end_matches('/');
    let base = s.rsplit('/').next().unwrap_or(s);
    if pol.deny_globs_and_roots && (s.contains('*') || matches!(s.trim(), "." | "/" | "~")) {
        return true;
    }
    if base.contains('.') {
        return false;
    }
    if pol.deny_extensionless_basename {
        return true;
    }
    for suf in &pol.tree_basename_suffixes {
        if !suf.is_empty() && s.ends_with(suf.as_str()) {
            return true;
        }
    }
    s.contains('/') && !base.contains('.')
}
