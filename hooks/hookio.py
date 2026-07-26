#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
LOG_DIR = Path(os.environ.get("KLEOS_STATE_DIR") or (Path.home() / ".cursor" / "hooks-state"))

CONTENT_KEYS = (
    "contents",
    "content",
    "new_string",
    "old_string",
    "cell_content",
    "new_source",
    "command",
    "cmd",
)


def load_stdin() -> dict:
    try:
        raw = sys.stdin.read()
        if not raw.strip():
            return {}
        return json.loads(raw)
    except Exception:
        return {"_parse_error": True}


def emit(obj: dict, code: int = 0) -> None:
    try:
        sys.stdout.write(json.dumps(obj, ensure_ascii=False) + "\n")
        sys.stdout.flush()
    except BrokenPipeError:
        pass
    raise SystemExit(code)


def deny(msg: str, code: int = 2) -> None:
    emit({"permission": "deny", "user_message": msg, "agent_message": msg}, code)


def ask(msg: str) -> None:
    emit({"permission": "ask", "user_message": msg, "agent_message": msg}, 0)


def allow() -> None:
    emit({"permission": "allow"}, 0)


def walk_strings(obj: Any) -> list[str]:
    out: list[str] = []

    def rec(x: Any) -> None:
        if isinstance(x, str):
            if x.strip():
                out.append(x)
            return
        if isinstance(x, dict):
            for v in x.values():
                rec(v)
            return
        if isinstance(x, (list, tuple)):
            for v in x:
                rec(v)

    rec(obj)
    return out


def content_blobs(obj: Any) -> list[str]:
    out: list[str] = []

    def rec(x: Any) -> None:
        if isinstance(x, dict):
            for k, v in x.items():
                if k in CONTENT_KEYS and isinstance(v, str) and v.strip():
                    out.append(v)
                elif k == "edits" and isinstance(v, list):
                    for ed in v:
                        rec(ed)
                else:
                    rec(v)
            return
        if isinstance(x, (list, tuple)):
            for v in x:
                rec(v)

    rec(obj)
    return out


def tool_input(data: dict) -> dict:
    inp = data.get("tool_input") or data.get("input") or {}
    return inp if isinstance(inp, dict) else {}


def path_from(data: dict) -> str:
    inp = tool_input(data)
    for k in ("path", "file_path", "filePath", "target_notebook", "uri"):
        v = inp.get(k) or data.get(k)
        if isinstance(v, str) and v.strip():
            return v
    return ""


def command_from(data: dict) -> str:
    for k in ("command", "cmd"):
        v = data.get(k)
        if isinstance(v, str):
            return v
    inp = tool_input(data)
    for k in ("command", "cmd"):
        v = inp.get(k)
        if isinstance(v, str):
            return v
    return ""


def conversation_id(data: dict) -> str:
    return str(data.get("conversation_id") or data.get("session_id") or "unknown")


def parse_maybe_json(text: str) -> Any:
    if not isinstance(text, str) or not text.strip():
        return None
    try:
        return json.loads(text)
    except Exception:
        pass
    try:
        return json.loads(re.sub(r"[\r\n]+", " ", text))
    except Exception:
        return None


def mcp_blobs(data: dict) -> list[str]:
    blobs = walk_strings(data)
    for key in ("tool_input", "input", "arguments", "args"):
        v = data.get(key)
        if isinstance(v, str):
            parsed = parse_maybe_json(v)
            if parsed is not None:
                blobs.extend(walk_strings(parsed))
            else:
                blobs.append(v)
        elif isinstance(v, dict):
            blobs.extend(walk_strings(v))
    return blobs


def _ensure_dir() -> Path:
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    return LOG_DIR


def log_decision(gate: str, event: str, permission: str, detail: str = "") -> None:
    try:
        path = _ensure_dir() / "obedience.jsonl"
        row = {
            "ts": datetime.now(timezone.utc).isoformat(),
            "gate": gate,
            "event": event,
            "permission": permission,
            "detail": detail[:240],
        }
        with path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(row, ensure_ascii=False) + "\n")
    except Exception:
        pass


def event_log_path(cid: str) -> Path:
    safe = re.sub(r"[^A-Za-z0-9._-]+", "_", cid or "unknown")[:120]
    return _ensure_dir() / f"events-{safe}.jsonl"


def append_event(cid: str, kind: str, **fields: Any) -> None:
    try:
        row = {
            "ts": datetime.now(timezone.utc).isoformat(),
            "kind": kind,
            **{k: v for k, v in fields.items() if v is not None},
        }
        line = json.dumps(row, ensure_ascii=False) + "\n"
        with event_log_path(cid).open("a", encoding="utf-8") as f:
            f.write(line)
    except Exception:
        pass


def read_events(cid: str) -> list[dict]:
    p = event_log_path(cid)
    if not p.is_file():
        return []
    out: list[dict] = []
    try:
        for line in p.read_text(encoding="utf-8", errors="replace").splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                out.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    except Exception:
        return []
    return out


def freshness(cid: str) -> dict:
    events = read_events(cid)
    dirty: dict[str, str] = {}
    last_verify = ""
    tools = 0
    loops = 0
    for ev in events:
        kind = ev.get("kind")
        if kind == "tool":
            tools += 1
        elif kind == "edit":
            path = str(ev.get("path") or "")
            if path:
                dirty[path] = str(ev.get("ts") or "")
        elif kind == "verify":
            last_verify = str(ev.get("ts") or "")
            dirty.clear()
        elif kind == "deny_repeat":
            loops += 1
        elif kind == "compact":
            pass
    unverified = sorted(dirty.keys())
    return {
        "tools": tools,
        "loops": loops,
        "last_verify": last_verify,
        "unverified": unverified,
        "dirty": dirty,
    }


def fingerprint(payload: dict) -> str:
    blob = json.dumps(payload, sort_keys=True, ensure_ascii=False, default=str)
    return hashlib.sha256(blob.encode("utf-8", "replace")).hexdigest()[:24]


def deny_fp_path(cid: str) -> Path:
    safe = re.sub(r"[^A-Za-z0-9._-]+", "_", cid or "unknown")[:120]
    return _ensure_dir() / f"denies-{safe}.jsonl"


def record_deny_fp(cid: str, fp: str) -> int:
    path = deny_fp_path(cid)
    count = 0
    try:
        if path.is_file():
            for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
                if fp in line:
                    count += 1
        with path.open("a", encoding="utf-8") as f:
            f.write(json.dumps({"ts": datetime.now(timezone.utc).isoformat(), "fp": fp}) + "\n")
        count += 1
    except Exception:
        return 1
    return count


def prior_deny_count(cid: str, fp: str) -> int:
    path = deny_fp_path(cid)
    if not path.is_file():
        return 0
    n = 0
    try:
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            if fp in line:
                n += 1
    except Exception:
        return 0
    return n


def ledger_path(conversation_id: str) -> Path:
    safe = re.sub(r"[^A-Za-z0-9._-]+", "_", conversation_id or "unknown")[:120]
    return _ensure_dir() / f"ledger-{safe}.json"


def read_ledger(conversation_id: str) -> dict:
    return freshness(conversation_id)


def write_ledger(conversation_id: str, data: dict) -> None:
    return None
