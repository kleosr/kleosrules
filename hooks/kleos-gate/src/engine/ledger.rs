use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::fs::{self, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn safe_cid(cid: &str) -> String {
    let mut out = String::new();
    for c in cid.chars().take(120) {
        if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "unknown".into()
    } else {
        out
    }
}

fn now_iso() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

pub fn conversation_id(data: &Value) -> String {
    data.get("conversation_id")
        .or_else(|| data.get("session_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}

fn event_log_path(state: &Path, cid: &str) -> PathBuf {
    state.join(format!("events-{}.jsonl", safe_cid(cid)))
}

fn deny_fp_path(state: &Path, cid: &str) -> PathBuf {
    state.join(format!("denies-{}.jsonl", safe_cid(cid)))
}

pub fn append_event(state: &Path, cid: &str, kind: &str, fields: Value) {
    let ignored = fs::create_dir_all(state);
    drop(ignored);
    let mut row = json!({"ts": now_iso(), "kind": kind});
    if let Some(obj) = fields.as_object() {
        if let Some(map) = row.as_object_mut() {
            for (k, v) in obj {
                map.insert(k.clone(), v.clone());
            }
        }
    }
    let line = format!("{row}\n");
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(event_log_path(state, cid))
    {
        let ignored2 = f.write_all(line.as_bytes());
        drop(ignored2);
    }
}

pub fn read_events(state: &Path, cid: &str) -> Vec<Value> {
    let raw = fs::read_to_string(event_log_path(state, cid)).unwrap_or_default();
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            out.push(v);
        }
    }
    out
}

pub struct Freshness {
    pub tools: usize,
    pub loops: usize,
    pub unverified: Vec<String>,
}

pub fn freshness(state: &Path, cid: &str) -> Freshness {
    let events = read_events(state, cid);
    let mut dirty: Vec<(String, String)> = Vec::new();
    let mut tools = 0usize;
    let mut loops = 0usize;
    for ev in events {
        let kind = ev.get("kind").and_then(|v| v.as_str()).unwrap_or("");
        match kind {
            "tool" => tools += 1,
            "edit" => {
                if let Some(path) = ev.get("path").and_then(|v| v.as_str()) {
                    if !path.is_empty() {
                        let ts = ev
                            .get("ts")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        dirty.retain(|(p, _)| p != path);
                        dirty.push((path.to_string(), ts));
                    }
                }
            }
            "verify" => dirty.clear(),
            "deny_repeat" => loops += 1,
            _ => {}
        }
    }
    let mut unverified: Vec<String> = dirty.into_iter().map(|(p, _)| p).collect();
    unverified.sort();
    Freshness {
        tools,
        loops,
        unverified,
    }
}

pub fn fingerprint(payload: &Value) -> String {
    let blob = serde_json::to_string(payload).unwrap_or_default();
    let mut h = DefaultHasher::new();
    blob.hash(&mut h);
    format!("{:016x}", h.finish())
}

pub fn prior_deny_count(state: &Path, cid: &str, fp: &str) -> usize {
    let raw = fs::read_to_string(deny_fp_path(state, cid)).unwrap_or_default();
    raw.lines().filter(|l| l.contains(fp)).count()
}

pub fn record_deny_fp(state: &Path, cid: &str, fp: &str) -> usize {
    let ignored = fs::create_dir_all(state);
    drop(ignored);
    let count = prior_deny_count(state, cid, fp) + 1;
    let line = format!("{}\n", json!({"ts": now_iso(), "fp": fp}));
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(deny_fp_path(state, cid))
    {
        let ignored2 = f.write_all(line.as_bytes());
        drop(ignored2);
    }
    count
}
