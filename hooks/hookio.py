#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable

HERE = Path(__file__).resolve().parent
LOG_DIR = Path.home() / ".cursor" / "hooks-state"


def load_stdin() -> dict:
    try:
        raw = sys.stdin.read()
        if not raw.strip():
            return {}
        return json.loads(raw)
    except Exception:
        return {"_parse_error": True}


def emit(obj: dict, code: int = 0) -> None:
    print(json.dumps(obj, ensure_ascii=False))
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


def parse_maybe_json(text: str) -> Any:
    if not isinstance(text, str) or not text.strip():
        return None
    try:
        return json.loads(text)
    except Exception:
        pass
    fixed = text
    try:
        fixed = re.sub(r"[\r\n]+", " ", text)
        return json.loads(fixed)
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


def log_decision(gate: str, event: str, permission: str, detail: str = "") -> None:
    try:
        LOG_DIR.mkdir(parents=True, exist_ok=True)
        path = LOG_DIR / "obedience.jsonl"
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


def ledger_path(conversation_id: str) -> Path:
    safe = re.sub(r"[^A-Za-z0-9._-]+", "_", conversation_id or "unknown")[:120]
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    return LOG_DIR / f"ledger-{safe}.json"


def read_ledger(conversation_id: str) -> dict:
    p = ledger_path(conversation_id)
    if not p.is_file():
        return {"edits": 0, "verifies": 0, "tools": 0, "roof_every": 12}
    try:
        return json.loads(p.read_text(encoding="utf-8"))
    except Exception:
        return {"edits": 0, "verifies": 0, "tools": 0, "roof_every": 12}


def write_ledger(conversation_id: str, data: dict) -> None:
    try:
        ledger_path(conversation_id).write_text(
            json.dumps(data, ensure_ascii=False), encoding="utf-8"
        )
    except Exception:
        pass
