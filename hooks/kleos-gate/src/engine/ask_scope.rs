use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::policy::Policy;
use crate::{ask, deny, path_from};

const LEDGER: &str = "ask_scope_paths.txt";

pub fn record_prompt(data: &Value, policy: &Policy, state: &Path) {
    if !policy.ask_scope.enabled {
        return;
    }
    let mut blobs = Vec::new();
    crate::walk_strings(data, &mut blobs);
    let text = blobs.join("\n");
    let re = Regex::new(&policy.ask_scope.path_token_pattern).unwrap_or_else(|_| {
        deny("kleos-gate invalid ask-scope path_token_pattern")
    });
    let mut tokens: Vec<String> = Vec::new();
    for cap in re.captures_iter(&text) {
        if let Some(m) = cap.get(1) {
            let t = m.as_str().to_string();
            if !tokens.contains(&t) {
                tokens.push(t);
            }
        }
    }
    let dest = state.join(LEDGER);
    let body = tokens.join("\n");
    let ignored = fs::write(&dest, body);
    drop(ignored);
}

pub fn check_path(data: &Value, policy: &Policy, state: &Path) {
    if !policy.ask_scope.enabled {
        return;
    }
    let path = path_from(data);
    if path.is_empty() {
        return;
    }
    let norm = path.replace('\\', "/");
    for pref in &policy.ask_scope.exempt_prefixes {
        if norm.contains(pref) || norm.ends_with(pref.trim_end_matches('/')) {
            return;
        }
    }
    let ledger = state.join(LEDGER);
    let raw = fs::read_to_string(&ledger).unwrap_or_default();
    let tokens: Vec<&str> = raw
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if tokens.len() < policy.ask_scope.min_tokens {
        return;
    }
    let base = PathBuf::from(&norm)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    for t in &tokens {
        if norm.ends_with(t) || norm.contains(&format!("/{t}")) || base == *t || norm == *t {
            return;
        }
        if t.ends_with(&base) {
            return;
        }
    }
    if policy.ask_scope.mode == "deny" {
        deny(&policy.ask_scope.message);
    } else {
        ask(&policy.ask_scope.message);
    }
}
