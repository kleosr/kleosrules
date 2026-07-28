use regex::Regex;
use serde_json::Value;

use crate::policy::{LeanPolicy, Policy};
use crate::{ask, allow, command_from, deny};
use super::{lean, prose, vernacular};

pub fn run(data: &Value, policy: &Policy) {
    let cmd = command_from(data);
    if cmd.is_empty() {
        allow();
    }
    for rule in &policy.shell.deny {
        let re = Regex::new(&format!("(?i){}", rule.pattern)).unwrap_or_else(|_| {
            deny("kleos-gate invalid shell deny regex in policy")
        });
        if re.is_match(&cmd) {
            deny(&rule.message);
        }
    }
    if looks_like_prose_shell_write(&cmd) {
        deny(&policy.shell.prose_shell_deny_message);
    }
    if let Some((path, body)) = embedded_code_write(&cmd, &policy.lean) {
        if prose::has_prose(&body) {
            deny(&policy.shell.prose_shell_deny_message);
        }
        if lean::is_code_path(&path, &policy.lean) {
            if let Err(reason) = vernacular::check_body_and_path(&path, &body) {
                deny(&reason);
            }
            if let Some(reason) = lean::check(&path, Some(&body), None, None, &policy.lean) {
                deny(&reason);
            }
            deny_shell_file_write(policy);
        }
    }
    for rule in &policy.shell.ask {
        let re = Regex::new(&format!("(?i){}", rule.pattern)).unwrap_or_else(|_| {
            deny("kleos-gate invalid shell ask regex in policy")
        });
        if re.is_match(&cmd) {
            ask(&rule.message);
        }
    }
    if looks_opaque_write(&cmd, &policy.lean) && !is_patch_or_apply(&cmd) {
        deny_shell_file_write(policy);
    }
    if looks_opaque_write(&cmd, &policy.lean) && is_patch_or_apply(&cmd) {
        let msg = &policy.shell.opaque_write_ask_message;
        if !msg.is_empty() {
            ask(msg);
        }
    }
    allow();
}

fn deny_shell_file_write(policy: &Policy) -> ! {
    let msg = policy.shell.opaque_write_deny_message.trim();
    if msg.is_empty() {
        deny("Blocked shell write of app files. Use Cursor Write/StrReplace only.");
    }
    deny(msg);
}

fn is_patch_or_apply(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    lower.contains("git apply")
        || lower.contains("git am ")
        || (lower.contains("patch ") && !lower.contains("dispatch"))
}

fn scrub_fd_redirects(cmd: &str) -> String {
    cmd.replace("2>&1", " ")
        .replace(">&1", " ")
        .replace(">&2", " ")
        .replace("&>", " ")
}

fn has_code_ext(cmd: &str, pol: &LeanPolicy) -> bool {
    let lower = cmd.to_lowercase();
    let mut exts: Vec<&String> = pol.code_extensions.iter().collect();
    exts.sort_by_key(|e| std::cmp::Reverse(e.len()));
    for ext in exts {
        let e = ext.to_lowercase();
        let mut from = 0usize;
        while let Some(rel) = lower[from..].find(&e) {
            let at = from + rel;
            let after = at + e.len();
            let ok_after = after >= lower.len()
                || !lower.as_bytes().get(after).is_some_and(|b| b.is_ascii_alphanumeric());
            if ok_after {
                return true;
            }
            from = at + 1;
        }
    }
    false
}

fn looks_like_prose_shell_write(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    if !(lower.contains(".ts")
        || lower.contains(".js")
        || lower.contains(".py")
        || lower.contains(".rs")
        || lower.contains(".go")
        || lower.contains(".css"))
    {
        return false;
    }
    let has_line = cmd.contains("//");
    let has_block = cmd.contains("/*");
    if !has_line && !has_block {
        return false;
    }
    if lower.contains("tee ") || cmd.contains(">>") {
        return true;
    }
    scrub_fd_redirects(cmd).contains('>')
}

fn looks_opaque_write(cmd: &str, pol: &LeanPolicy) -> bool {
    let lower = cmd.to_lowercase();
    if lower.contains("sed -i")
        || lower.contains("git apply")
        || lower.contains("patch ")
    {
        return true;
    }
    if !has_code_ext(cmd, pol) {
        return false;
    }
    if lower.contains("tee ") || lower.contains("<<") {
        return true;
    }
    scrub_fd_redirects(cmd).contains('>')
}

fn embedded_code_write(cmd: &str, pol: &LeanPolicy) -> Option<(String, String)> {
    let heredoc_at = cmd.find("<<")?;
    let before = cmd[..heredoc_at].trim_end();
    let path_re = Regex::new(r#">\s*([^\s;|&]+)$"#).ok()?;
    let path = path_re.captures(before)?.get(1)?.as_str().to_string();
    if !lean::is_code_path(&path, pol) {
        return None;
    }
    let mut rest = cmd[heredoc_at + 2..].trim_start();
    let quote = rest.chars().next().filter(|c| *c == '\'' || *c == '"');
    if quote.is_some() {
        rest = &rest[1..];
    }
    let tag_end = rest.find(|c: char| c.is_whitespace() || c == '\'' || c == '"')?;
    let tag = &rest[..tag_end];
    if tag.is_empty() {
        return None;
    }
    rest = &rest[tag_end..];
    if let Some(q) = quote {
        rest = rest.strip_prefix(q).unwrap_or(rest);
    }
    rest = rest.strip_prefix('\r').unwrap_or(rest);
    rest = rest.strip_prefix('\n').unwrap_or(rest);
    let closer = format!("\n{tag}");
    let closer_cr = format!("\r\n{tag}");
    let body = if let Some(i) = rest.find(&closer_cr) {
        &rest[..i]
    } else if let Some(i) = rest.find(&closer) {
        &rest[..i]
    } else if rest.ends_with(tag) && rest.len() >= tag.len() {
        let b = &rest[..rest.len() - tag.len()];
        b.strip_suffix('\n').or_else(|| b.strip_suffix("\r\n")).unwrap_or(b)
    } else {
        return None;
    };
    Some((path, body.to_string()))
}
