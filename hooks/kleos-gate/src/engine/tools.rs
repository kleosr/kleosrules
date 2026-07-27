use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use crate::engine::prose;
use crate::policy::Policy;

fn pack_root(hooks: &Path) -> PathBuf {
    if hooks.file_name().and_then(|s| s.to_str()) == Some("hooks") {
        hooks
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| hooks.to_path_buf())
    } else {
        hooks.to_path_buf()
    }
}

fn skip_dir(name: &str) -> bool {
    matches!(
        name,
        "target" | ".git" | "node_modules" | "__pycache__" | ".venv" | "dist" | "build"
    )
}

fn walk_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(root) else {
        return;
    };
    for ent in rd.flatten() {
        let path = ent.path();
        let name = ent.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if skip_dir(&name) {
                continue;
            }
            walk_files(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

pub fn gate_diff(hooks: &Path, policy: &Policy) -> ! {
    let root = pack_root(hooks);
    let mut files = Vec::new();
    walk_files(&root, &mut files);
    let mut fails: Vec<String> = Vec::new();
    for path in files {
        let rel = path
            .strip_prefix(&root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| path.to_string_lossy().into());
        if !policy.lean.code_extensions.iter().any(|ext| {
            let e = ext.to_lowercase();
            if matches!(
                e.as_str(),
                ".sh" | ".bash" | ".zsh" | ".ps1" | ".fish" | ".py" | ".pyi"
            ) {
                return false;
            }
            rel.to_lowercase().ends_with(&e)
        }) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if prose::has_prose(&text) {
            fails.push(rel);
        }
    }
    if fails.is_empty() {
        let ignored = writeln!(io::stdout(), "GATE_DIFF_PASS");
        drop(ignored);
        process::exit(0);
    }
    for f in &fails {
        let ignored = writeln!(io::stdout(), "[FAIL] prose comment: {f}");
        drop(ignored);
    }
    let ignored = writeln!(
        io::stdout(),
        "[FAIL] {} file(s) with prose comments",
        fails.len()
    );
    drop(ignored);
    process::exit(1);
}

pub fn obedience_report(state: &Path) -> ! {
    let log = state.join("obedience.jsonl");
    if !log.is_file() {
        let ignored = writeln!(io::stdout(), "no obedience log yet: {}", log.display());
        drop(ignored);
        process::exit(0);
    }
    let raw = fs::read_to_string(&log).unwrap_or_default();
    let mut counts: HashMap<(String, String), usize> = HashMap::new();
    let mut n = 0usize;
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        n += 1;
        let gate = v
            .get("gate")
            .and_then(|x| x.as_str())
            .unwrap_or("?")
            .to_string();
        let perm = v
            .get("permission")
            .and_then(|x| x.as_str())
            .unwrap_or("?")
            .to_string();
        *counts.entry((gate, perm)).or_insert(0) += 1;
    }
    if n == 0 {
        let ignored = writeln!(io::stdout(), "empty obedience log");
        drop(ignored);
        process::exit(0);
    }
    let ignored = writeln!(io::stdout(), "decisions={n}");
    drop(ignored);
    let mut keys: Vec<_> = counts.keys().cloned().collect();
    keys.sort();
    for (gate, perm) in keys {
        let c = counts[&(gate.clone(), perm.clone())];
        let rate = 100.0 * (c as f64) / (n as f64);
        let ignored = writeln!(io::stdout(), "{gate}\t{perm}\t{c}\t{rate:.1}/100");
        drop(ignored);
    }
    process::exit(0);
}

pub fn check_user_rules(hooks: &Path) -> ! {
    let root = pack_root(hooks);
    let paste = root.join("user-rules/USER-RULES.paste.txt");
    let option = env_home()
        .join(".cursor")
        .join("rules")
        .join("option-c-core.mdc");
    let markers = [
        "Master Mind V15",
        "PRIME OBEDIENCE",
        "MUST-NEVER",
        "NO COMMENTS",
        "VERNACULAR",
        "kleos-gate",
        "PRE-FLIGHT",
        "MECHANICAL GATES",
    ];
    let mut hits = 0usize;
    for path in [&paste, &option] {
        if let Ok(text) = fs::read_to_string(path) {
            hits += markers.iter().filter(|m| text.contains(*m)).count();
        }
    }
    if hits >= 6 {
        process::exit(0);
    }
    let ignored = writeln!(
        io::stderr(),
        "user-rules markers weak (hits={hits}); paste or option-c-core missing V15 signals"
    );
    drop(ignored);
    process::exit(1);
}

fn env_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"))
}
