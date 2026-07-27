use serde_json::Value;

const MIN_DURABLE_CHARS: usize = 280;
const MIN_SOURCE_CHARS: usize = 400;

fn lower_blob(v: &Value) -> String {
    fn walk(v: &Value, out: &mut String) {
        match v {
            Value::String(s) => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(s);
            }
            Value::Array(a) => {
                for x in a {
                    walk(x, out);
                }
            }
            Value::Object(m) => {
                for (_k, x) in m {
                    walk(x, out);
                }
            }
            _ => {}
        }
    }
    let mut out = String::new();
    walk(v, &mut out);
    out
}

fn find_string_key(v: &Value, keys: &[&str]) -> Option<String> {
    match v {
        Value::Object(m) => {
            for k in keys {
                if let Some(Value::String(s)) = m.get(*k) {
                    if !s.is_empty() {
                        return Some(s.clone());
                    }
                }
            }
            for (_k, child) in m {
                if let Some(s) = find_string_key(child, keys) {
                    return Some(s);
                }
            }
            None
        }
        Value::Array(a) => {
            for child in a {
                if let Some(s) = find_string_key(child, keys) {
                    return Some(s);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn vault_path(input: &Value) -> String {
    find_string_key(input, &["path", "file_path", "filePath"]).unwrap_or_default()
}

pub fn vault_body(input: &Value) -> String {
    find_string_key(input, &["content", "contents", "text", "markdown", "new_string"])
        .unwrap_or_else(|| lower_blob(input))
}

pub fn is_meta_path(path: &str) -> bool {
    let p = path.replace('\\', "/").to_ascii_lowercase();
    p.contains("wiki/hot.md")
        || p.contains("wiki/index.md")
        || p.contains("wiki/log.md")
        || p.contains("wiki/catalogs/")
        || p.contains("instructions/")
}

pub fn is_durable_path(path: &str) -> bool {
    let p = path.replace('\\', "/").to_ascii_lowercase();
    p.contains("wiki/sources/")
        || p.contains("/sessions/")
        || p.contains("decisions.md")
        || p.contains("learnings.md")
        || p.contains("wiki/concepts/")
        || p.contains("wiki/entities/")
        || p.contains("wiki/journals/")
}

fn has_any(hay: &str, needles: &[&str]) -> bool {
    let low = hay.to_ascii_lowercase();
    needles.iter().any(|n| low.contains(&n.to_ascii_lowercase()))
}

pub fn is_session_path(path: &str) -> bool {
    path.replace('\\', "/").to_ascii_lowercase().contains("/sessions/")
}

pub fn has_intent_markers(body: &str) -> bool {
    let goal = has_any(body, &["## goal", "# goal", "**goal**", "goal\n", "goal:"]);
    let done = has_any(
        body,
        &[
            "done-when",
            "done when",
            "## done",
            "done-when:",
            "**done-when**",
        ],
    );
    let residual = has_any(body, &["## residual", "residual:", "**residual**"]);
    goal && (done || residual)
}

pub fn has_layer_check(body: &str) -> bool {
    has_any(
        body,
        &[
            "layer check",
            "layer_check",
            "## layer check",
            "prompt | context | harness",
            "| prompt |",
            "layer: prompt",
        ],
    )
}

pub fn is_complete_body(path: &str, body: &str) -> bool {
    if is_meta_path(path) {
        return true;
    }
    if !is_durable_path(path) {
        return true;
    }
    let chars = body.chars().count();
    let p = path.replace('\\', "/").to_ascii_lowercase();
    if p.contains("wiki/sources/") {
        return chars >= MIN_SOURCE_CHARS;
    }
    if is_session_path(path) {
        return chars >= MIN_DURABLE_CHARS && has_intent_markers(body);
    }
    if p.contains("decisions.md") {
        return chars >= 120
            && has_any(body, &["why", "residual", "options", "choice"]);
    }
    if p.contains("learnings.md") {
        return chars >= 80;
    }
    chars >= MIN_DURABLE_CHARS
}

#[derive(Debug, Clone, Default)]
pub struct PersistFlags {
    pub complete: bool,
    pub stub: bool,
    pub intent: bool,
    pub layer: bool,
}

pub fn classify_persist(input: &Value) -> PersistFlags {
    let path = vault_path(input);
    let body = vault_body(input);
    if path.is_empty() && body.is_empty() {
        return PersistFlags::default();
    }
    let mut flags = PersistFlags::default();
    if is_session_path(&path) || path.is_empty() {
        if has_intent_markers(&body) {
            flags.intent = true;
        }
        if has_layer_check(&body) {
            flags.layer = true;
        }
    } else {
        if has_intent_markers(&body) {
            flags.intent = true;
        }
        if has_layer_check(&body) {
            flags.layer = true;
        }
    }
    if path.is_empty() {
        if body.chars().count() >= MIN_DURABLE_CHARS {
            flags.complete = true;
        } else if !body.is_empty() {
            flags.stub = true;
        }
        return flags;
    }
    if is_complete_body(&path, &body) {
        flags.complete = true;
    } else if is_durable_path(&path) {
        flags.stub = true;
    } else {
        flags.complete = true;
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn session_stub_detected() {
        let input = json!({
            "server": "user-obsidian",
            "toolName": "vault_append",
            "arguments": {
                "path": "wiki/projects/kleosr/Sessions/2026-07-27-x.md",
                "content": "## Goal\nshort\n"
            }
        });
        let f = classify_persist(&input);
        assert!(f.stub || !f.complete);
    }

    #[test]
    fn session_complete_with_markers() {
        let body = r#"
## Goal
Ship five-layer E2E loop.

## Done-when
cargo test green; stop followups wired.

## What ran
Edited session.rs and ledger.rs.

## Evidence
cargo test -p kleos-gate

## Outcomes
Ledger flags + stop followups.

## Open
None.

## Residual
Heuristic ≠ semantic completeness.

## Layer check
| Layer | Evidence |
| Prompt | Intent restated |
| Context | vault recall |
| Harness | cargo test |
| Loop | Session note |
| Graph | Decisions wikilink |
"#;
        let input = json!({
            "arguments": {
                "path": "wiki/projects/kleosr/Sessions/2026-07-27-x.md",
                "content": body
            }
        });
        let f = classify_persist(&input);
        assert!(f.complete, "{f:?}");
        assert!(f.intent);
        assert!(f.layer);
        assert!(!f.stub);
    }

    #[test]
    fn hot_meta_is_complete() {
        let input = json!({
            "arguments": {
                "path": "wiki/hot.md",
                "content": "- tip\n"
            }
        });
        let f = classify_persist(&input);
        assert!(f.complete);
        assert!(!f.stub);
    }
}
