use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::OnceLock;

use super::capture;
use super::context;
use super::injection;
use super::ledger;
use crate::policy::Policy;
use crate::{allow_empty_prompt, command_from, emit, path_from, tool_input, walk_strings};

const ROOF: &str = "Master Mind roof: NO prose comments; no remote publish without confirmation; verify before Done; never fight a deny.";
const ROOF_EVERY: usize = 12;
const FOLLOW: &str = "Session has unverified edits. ACT NOW: run the house gauntlet yourself (TOOLCHAIN.md / tests / lint / kleos-gate verify) and cite evidence. Never ask the human to waive verification. Do not claim Done without verification evidence or a named residual.";
const LOOP_MSG: &str = "Freeze loop detected (repeat deny fingerprints). Stop retrying the same blocked write; rewrite to an allowed surface or ask the user.";
const OBSIDIAN_RECALL: &str = "Obsidian memory MANDATORY before substantive work: GetMcpTools user-obsidian → vault_read wiki/hot.md then wiki/index.md then wiki/projects/<slug>/Index.md + latest Sessions/. Query wiki only; never edit raw/. Skill obsidian-memory.";
const OBSIDIAN_FLUSH: &str = "Obsidian write-back required (persist INTO vault — never wipe): session had tool work but no vault write logged. vault_append/vault_write wiki/projects/<slug>/Decisions|Learnings|Sessions/YYYY-MM-DD-<topic>.md + refresh wiki/hot.md; mirror HANDOFF.md. Skill obsidian-memory.";
const OBSIDIAN_COMPACT: &str = "Compaction imminent — write-back TO Obsidian NOW (Session + Decisions/Learnings via user-obsidian) before chat context dies. This saves memory; it does not clear the vault.";
const OBSIDIAN_DUTY: &str = "Obsidian duty: if this block produced a durable decision/learning/landmine, vault_append it now with [[wikilinks]] (wiki/ only; never edit raw/).";
const OBSIDIAN_STUB: &str = "COMPLETE CAPTURE: vault write looks thin/stub. Expand durable pages (Sessions/Decisions/Learnings/sources/concepts) to full depth — Goal/Done-when/Residual + body enough to act without chat. instructions/PROCESSING.";
const INTENT_FLUSH: &str = "INTENT required: restate verified intent + done-when in chat and wiki/projects/<slug>/Sessions/ (Goal + Done-when + Residual). Do not rewrite the user prompt — restate beside it.";
const LAYER_FLUSH: &str = "LAYER CHECK required in Session before Done: prompt|context|harness|loop|graph with evidence each. docs/CURSOR-CURATOR.md · docs/LAYER-STACK.md.";

fn verify_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r"(?i)(^|[;&|]\s*)(npm\s+test|pnpm\s+test|yarn\s+test|bun\s+test|pytest|python\s+-m\s+pytest|cargo\s+test|go\s+test|tsc(\s|$)|ruff\s+check|eslint|gradlew?\s+test|gradlew?\s+check|make\s+test|cmake\s+--build|bash\s+.*TOOLCHAIN|npm\s+run\s+(test|lint|check|typecheck|build)|pnpm\s+run\s+(test|lint|check|typecheck|build)|yarn\s+run\s+(test|lint|check|typecheck|build)|(?:hooks/bin/)?kleos-gate\s+(verify|bench|gate-diff|test)|npm\s+run\s+check:domain)",
        )
        .expect("verify regex")
    })
}

fn is_edit_tool(tool: &str) -> bool {
    matches!(
        tool,
        "Write" | "StrReplace" | "EditNotebook" | "EditFile" | "Delete" | "ApplyPatch"
    ) || tool.contains("Write")
        || tool.contains("StrReplace")
        || tool.contains("Edit")
        || tool.contains("Delete")
}

fn obsidian_blob(data: &Value) -> String {
    let mut blobs = Vec::new();
    if let Some(t) = data
        .get("tool_name")
        .or_else(|| data.get("toolName"))
        .and_then(|v| v.as_str())
    {
        blobs.push(t.to_string());
    }
    walk_strings(tool_input(data), &mut blobs);
    walk_strings(data, &mut blobs);
    blobs.join("\n").to_lowercase()
}

fn is_obsidian_server(t: &str) -> bool {
    t.contains("user-obsidian") || t.contains("\"obsidian\"") || t.contains("server\":\"obsidian")
}

fn is_obsidian_persist(data: &Value) -> bool {
    let t = obsidian_blob(data);
    let write_hit = t.contains("vault_write")
        || t.contains("vault_append")
        || t.contains("vault_patch");
    is_obsidian_server(&t) && write_hit
}

fn is_obsidian_recall(data: &Value) -> bool {
    let t = obsidian_blob(data);
    if !is_obsidian_server(&t) {
        return false;
    }
    let search_hit = t.contains("search_simple") || t.contains("search_query");
    let read_hit = t.contains("vault_read")
        && (t.contains("wiki/hot") || t.contains("wiki/index"));
    search_hit || read_hit
}

fn tool_result_text(data: &Value) -> String {
    let mut parts = Vec::new();
    for key in ["result", "output", "content", "text", "stdout", "stderr"] {
        if let Some(v) = data.get(key) {
            walk_strings(v, &mut parts);
        }
    }
    if let Some(tr) = data.get("tool_result") {
        walk_strings(tr, &mut parts);
    }
    parts.join("\n")
}

fn mcp_output_text(data: &Value) -> String {
    for key in ["mcp_tool_output", "tool_output", "output"] {
        if let Some(v) = data.get(key) {
            if let Some(s) = v.as_str() {
                return s.to_string();
            }
            let mut parts = Vec::new();
            walk_strings(v, &mut parts);
            if !parts.is_empty() {
                return parts.join("\n");
            }
        }
    }
    String::new()
}

pub fn post_tool_use(data: &Value, state: &Path) {
    let cid = ledger::conversation_id(data);
    let tool = data
        .get("tool_name")
        .or_else(|| data.get("toolName"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    ledger::append_event(state, &cid, "tool", json!({"name": tool}));

    if is_edit_tool(tool) {
        let path = path_from(data);
        if !path.is_empty()
            && !path.ends_with(".md")
            && !path.ends_with(".mdc")
            && !path.ends_with(".txt")
            && !path.ends_with(".json")
        {
            ledger::append_event(state, &cid, "edit", json!({"path": path}));
        }
    }

    if is_obsidian_persist(data) {
        ledger::append_event(state, &cid, "obsidian", json!({}));
        let flags = capture::classify_persist(tool_input(data));
        if flags.complete {
            ledger::append_event(state, &cid, "obsidian_complete", json!({}));
        }
        if flags.stub {
            ledger::append_event(state, &cid, "obsidian_stub", json!({}));
        }
        if flags.intent {
            ledger::append_event(state, &cid, "intent_stated", json!({}));
        }
        if flags.layer {
            ledger::append_event(state, &cid, "layer_check", json!({}));
        }
    }
    if is_obsidian_recall(data) {
        ledger::append_event(state, &cid, "obsidian_recall", json!({}));
    }

    let cmd = command_from(data);
    if !cmd.is_empty() && verify_re().is_match(&cmd) {
        ledger::append_event(
            state,
            &cid,
            "verify",
            json!({"cmd": cmd.chars().take(200).collect::<String>()}),
        );
    }

    let mut contexts: Vec<String> = Vec::new();
    let inj_text = tool_result_text(data);
    if tool != "Shell"
        && !inj_text.is_empty()
        && injection::is_injection(&inj_text)
    {
        contexts.push(injection::notice(&inj_text));
    }
    let mcp_text = mcp_output_text(data);
    let mut out = json!({});
    if tool != "Shell"
        && !mcp_text.is_empty()
        && injection::is_injection(&mcp_text)
    {
        let n = injection::notice(&mcp_text);
        contexts.push(n.clone());
        out["updated_mcp_tool_output"] = json!(serde_json::to_string(&json!({
            "warning": n,
            "filtered": true
        }))
        .unwrap_or_default());
    }

    for ev in ledger::read_events(state, &cid).iter().rev().take(8) {
        if ev.get("normalized").and_then(|v| v.as_bool()) == Some(true) {
            contexts.push("Normalize write stripped prose comments from payload.".into());
            break;
        }
    }

    let fresh = ledger::freshness(state, &cid);
    if fresh.tools > 0 && fresh.tools % ROOF_EVERY == 0 {
        contexts.push(ROOF.into());
        if !fresh.obsidian {
            contexts.push(OBSIDIAN_DUTY.into());
        }
    }

    if !contexts.is_empty() {
        out["additional_context"] = json!(contexts.join(" "));
    }
    if out.as_object().map(|m| !m.is_empty()).unwrap_or(false) {
        emit(&out, 0);
    }
    emit(&json!({}), 0);
}

pub fn session_boundary(data: &Value, state: &Path, hook_event: &str, policy: &Policy) {
    let cid = ledger::conversation_id(data);
    if hook_event == "sessionStart" || hook_event == "session-boundary" {
        let fresh = ledger::freshness(state, &cid);
        let mut parts = vec![ROOF.to_string(), OBSIDIAN_RECALL.to_string()];
        parts.extend(context::session_seed(&policy.context));
        if !fresh.unverified.is_empty() {
            parts.push(format!(
                "Session carry-over unverified paths: {}. ACT NOW: run verify-class commands yourself before claiming Done.",
                fresh.unverified.join(", ")
            ));
        }
        emit(&json!({"additional_context": parts.join(" ")}), 0);
    }
    if hook_event == "preCompact" {
        ledger::append_event(state, &cid, "compact", json!({}));
        let fresh = ledger::freshness(state, &cid);
        let mut msg = OBSIDIAN_COMPACT.to_string();
        if !fresh.unverified.is_empty() {
            msg.push_str(&format!(
                " Unverified edits on: {}. Re-verify after resume.",
                fresh.unverified.join(", ")
            ));
        }
        emit(
            &json!({
                "user_message": msg,
                "additional_context": OBSIDIAN_COMPACT
            }),
            0,
        );
    }
    allow_empty_prompt();
}

pub fn stop_verify(data: &Value, state: &Path) {
    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
    if !status.is_empty() && status != "completed" {
        allow_empty_prompt();
    }
    let cid = ledger::conversation_id(data);
    let fresh = ledger::freshness(state, &cid);
    let mut parts: Vec<String> = Vec::new();
    if fresh.loops >= 2 {
        parts.push(format!(
            "{} Unverified: {}",
            LOOP_MSG,
            fresh.unverified.join(", ")
        ));
    }
    if !fresh.unverified.is_empty() {
        let mut msg = format!(
            "{} Unverified paths: {}",
            FOLLOW,
            fresh.unverified.join(", ")
        );
        if !fresh.obsidian {
            msg.push(' ');
            msg.push_str(OBSIDIAN_FLUSH);
        }
        parts.push(msg);
    }
    if fresh.tools > 0 && !fresh.obsidian {
        parts.push(OBSIDIAN_FLUSH.to_string());
    } else if fresh.tools > 0 && fresh.obsidian_stub && !fresh.obsidian_complete {
        parts.push(OBSIDIAN_STUB.to_string());
    }
    if fresh.tools > 0 && !fresh.intent_stated {
        parts.push(INTENT_FLUSH.to_string());
    }
    if fresh.had_edits && !fresh.layer_check {
        parts.push(LAYER_FLUSH.to_string());
    }
    if !parts.is_empty() {
        parts.dedup();
        emit(
            &json!({ "followup_message": parts.join(" ") }),
            0,
        );
    }
    allow_empty_prompt();
}
