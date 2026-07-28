use regex::Regex;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::policy::{ClassifyRule, ContextPolicy};

pub fn vault_root(policy: &ContextPolicy) -> PathBuf {
    if let Ok(p) = env::var(&policy.vault_root_env) {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    PathBuf::from(&policy.vault_root)
}

pub fn hot_slice(policy: &ContextPolicy) -> Option<(String, usize, usize)> {
    let root = vault_root(policy);
    if !root.is_dir() {
        return None;
    }
    let path = root.join(&policy.hot_path);
    let raw = fs::read_to_string(&path).ok()?;
    if raw.is_empty() {
        return None;
    }
    let full = raw.chars().count();
    let body = if full <= policy.hot_chars_max {
        raw
    } else {
        let mut out = String::new();
        for c in raw.chars().take(policy.hot_chars_max) {
            out.push(c);
        }
        out.push_str("\n…[truncated]");
        out
    };
    let used = body.chars().count();
    Some((format!("wiki/hot.md (capped):\n{body}"), used, full))
}

const WEEKLY_LINT_MAX_AGE_SECS: u64 = 7 * 24 * 3600;
const WEEKLY_LINT_NUDGE: &str = "EVENT LOOP: weekly lint stale or missing (>7d). Run lint triad → wiki/audits/YYYY-MM-DD-weekly.md (template wiki/_templates/weekly-lint). Skill obsidian-memory.";

fn weekly_lint_stale(policy: &ContextPolicy) -> bool {
    let audits = vault_root(policy).join("wiki/audits");
    let Ok(rd) = fs::read_dir(&audits) else {
        return true;
    };
    let now = std::time::SystemTime::now();
    let mut freshest: Option<std::time::SystemTime> = None;
    for ent in rd.flatten() {
        let name = ent.file_name().to_string_lossy().to_lowercase();
        if !name.contains("weekly") {
            continue;
        }
        let Ok(meta) = ent.metadata() else {
            continue;
        };
        let Ok(mtime) = meta.modified() else {
            continue;
        };
        freshest = Some(match freshest {
            Some(prev) if prev > mtime => prev,
            _ => mtime,
        });
    }
    match freshest {
        None => true,
        Some(m) => now
            .duration_since(m)
            .map(|d| d.as_secs() > WEEKLY_LINT_MAX_AGE_SECS)
            .unwrap_or(true),
    }
}

pub fn session_seed(policy: &ContextPolicy) -> Vec<String> {
    let mut parts = Vec::new();
    if !policy.playbook.is_empty() {
        parts.push(policy.playbook.clone());
    }
    if let Some((hot, used, full)) = hot_slice(policy) {
        parts.push(hot);
        if policy.meter_enabled {
            parts.push(format!(
                "context-meter: hot_chars={used}/{cap} vault_full={full}",
                cap = policy.hot_chars_max
            ));
        }
    }
    if weekly_lint_stale(policy) {
        parts.push(WEEKLY_LINT_NUDGE.into());
    }
    parts
}

fn prompt_text(data: &Value) -> String {
    for k in ["prompt", "user_prompt", "message", "text"] {
        if let Some(s) = data.get(k).and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return s.to_string();
            }
        }
    }
    String::new()
}

fn is_stopword(tok: &str, policy: &ContextPolicy) -> bool {
    policy.pointer_stopwords.iter().any(|s| s == tok)
}

fn tokens(prompt: &str, policy: &ContextPolicy) -> Vec<String> {
    let min_len = policy.pointer_token_min_len;
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in prompt.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '/' || c == '.' {
            cur.push(c.to_ascii_lowercase());
        } else if !cur.is_empty() {
            if cur.len() >= min_len && !is_stopword(&cur, policy) {
                out.push(cur.clone());
            }
            cur.clear();
        }
    }
    if cur.len() >= min_len && !is_stopword(&cur, policy) {
        out.push(cur);
    }
    out.sort();
    out.dedup();
    out
}

fn candidate_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.contains("wiki/") || t.contains("[[") || t.starts_with('-') || t.starts_with('|') {
            out.push(t.to_string());
        }
    }
    out
}

fn score_line(line: &str, toks: &[String]) -> usize {
    let low = line.to_ascii_lowercase();
    toks.iter().filter(|t| low.contains(t.as_str())).count()
}

pub fn pointer_hits(prompt: &str, corpus: &str, policy: &ContextPolicy) -> Vec<String> {
    let toks = tokens(prompt, policy);
    if toks.is_empty() {
        return Vec::new();
    }
    let mut scored: Vec<(usize, String)> = candidate_lines(corpus)
        .into_iter()
        .map(|line| (score_line(&line, &toks), line))
        .filter(|(s, _)| *s >= policy.pointer_min_hits)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    scored.truncate(policy.pointer_max);
    scored.into_iter().map(|(_, line)| line).collect()
}

fn compile_rule(rule: &ClassifyRule) -> Option<Regex> {
    Regex::new(&rule.pattern).ok()
}

pub fn classify_hints(prompt: &str, policy: &ContextPolicy) -> Vec<String> {
    let mut out = Vec::new();
    for rule in &policy.classify_rules {
        if out.len() >= policy.classify_max {
            break;
        }
        let Some(re) = compile_rule(rule) else {
            continue;
        };
        if re.is_match(prompt) {
            out.push(format!("classify[{}]: {}", rule.id, rule.hint));
        }
    }
    out
}

pub fn ask_context(data: &Value, policy: &ContextPolicy) -> Option<String> {
    let prompt = prompt_text(data);
    if prompt.is_empty() {
        return None;
    }
    let mut parts = classify_hints(&prompt, policy);
    let root = vault_root(policy);
    if root.is_dir() {
        let mut corpus = String::new();
        if let Ok(index) = fs::read_to_string(root.join(&policy.index_path)) {
            corpus.push_str(&index);
            corpus.push('\n');
        }
        if let Ok(hot) = fs::read_to_string(root.join(&policy.hot_path)) {
            corpus.push_str(&hot);
        }
        let hits = pointer_hits(&prompt, &corpus, policy);
        if !hits.is_empty() {
            parts.push(format!(
                "Obsidian context pointers (drill via user-obsidian): {}",
                hits.join(" | ")
            ));
        }
    }
    if parts.is_empty() {
        return None;
    }
    Some(parts.join(" "))
}

pub fn is_write_exempt(path: &str, policy: &ContextPolicy) -> bool {
    let norm = path.replace('\\', "/");
    policy
        .exempt_write_prefixes
        .iter()
        .any(|p| norm.starts_with(p) || norm.contains(&format!("/{p}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::ContextPolicy;

    fn pol() -> ContextPolicy {
        ContextPolicy {
            vault_root_env: "KLEOS_VAULT".into(),
            vault_root: "/tmp/none".into(),
            hot_path: "wiki/hot.md".into(),
            index_path: "wiki/index.md".into(),
            hot_chars_max: 100,
            pointer_max: 3,
            pointer_min_hits: 1,
            pointer_token_min_len: 4,
            pointer_stopwords: vec!["that".into(), "this".into()],
            recall_gate_enabled: true,
            recall_message: "recall".into(),
            exempt_write_prefixes: vec![],
            meter_enabled: true,
            classify_max: 2,
            classify_rules: vec![ClassifyRule {
                id: "code".into(),
                pattern: r"(?i)\b(fix|implement)\b".into(),
                hint: "code-hint".into(),
            }],
            playbook: "PB".into(),
        }
    }

    #[test]
    fn classify_matches_code() {
        let hints = classify_hints("please fix the gate", &pol());
        assert_eq!(hints.len(), 1);
        assert!(hints[0].contains("code-hint"));
    }

    #[test]
    fn stopwords_drop_from_tokens() {
        let p = pol();
        let t = tokens("that this vault memory", &p);
        assert!(!t.iter().any(|x| x == "that"));
        assert!(t.iter().any(|x| x == "vault"));
    }
}