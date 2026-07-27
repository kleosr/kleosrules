use regex::Regex;
use serde_json::Value;

use crate::policy::Policy;
use crate::{ask, allow, command_from, deny};

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
    for rule in &policy.shell.ask {
        let re = Regex::new(&format!("(?i){}", rule.pattern)).unwrap_or_else(|_| {
            deny("kleos-gate invalid shell ask regex in policy")
        });
        if re.is_match(&cmd) {
            ask(&rule.message);
        }
    }
    if looks_opaque_write(&cmd) {
        ask(&policy.shell.opaque_write_ask_message);
    }
    allow();
}

fn looks_like_prose_shell_write(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    if !(lower.contains(".ts")
        || lower.contains(".js")
        || lower.contains(".py")
        || lower.contains(".rs")
        || lower.contains(".go"))
    {
        return false;
    }
    if cmd.contains("//") || cmd.contains("/*") {
        if cmd.contains(">>") || cmd.contains(">") || cmd.contains("tee ") {
            return true;
        }
    }
    false
}

fn looks_opaque_write(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    (lower.contains("sed -i")
        || lower.contains("git apply")
        || lower.contains("patch ")
        || lower.contains("tee "))
        && (lower.contains(".ts")
            || lower.contains(".js")
            || lower.contains(".py")
            || lower.contains(".rs"))
}
