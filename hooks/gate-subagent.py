#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import append_event, conversation_id, deny, emit, load_stdin  # noqa: E402
from injection_lib import is_injection, scan  # noqa: E402

GATED_ACTION = re.compile(
    r"(?:git\s+push[^;&|]*(?:--force| -f\s|--force-with-lease)|"
    r"git\s+reset\s+--hard|git\s+clean\s+-[a-z]*f|"
    r"rm\s+-[a-z]*rf|npm\s+publish|terraform\s+destroy|"
    r"curl[^\n]*\|\s*(?:sh|bash)|npm\s+install|npm\s+ci|"
    r"gh\s+release\s+create|docker\s+push|find\s+.*-delete)",
    re.I,
)

DENY_BRIEF = (
    "Blocked subagent brief with gated ASK-ONCE/MUST-NEVER actions or injection frames. "
    "Rewrite brief without force-push/publish/install/destructive commands or override frames."
)


def brief_text(data: dict) -> str:
    parts: list[str] = []
    for k in ("task", "brief", "prompt", "description", "message"):
        v = data.get(k)
        if isinstance(v, str) and v.strip():
            parts.append(v)
    sub = data.get("subagent") or data.get("agent")
    if isinstance(sub, dict):
        for k in ("task", "brief", "prompt", "description"):
            v = sub.get(k)
            if isinstance(v, str) and v.strip():
                parts.append(v)
    return "\n".join(parts)


def brief_denied(text: str) -> bool:
    if not text.strip():
        return False
    if is_injection(text):
        return True
    if GATED_ACTION.search(text):
        return True
    hit = scan(text)
    return bool(hit.actions)


def run(data: dict) -> None:
    event = str(data.get("hook_event_name") or "")
    cid = conversation_id(data)

    if event == "subagentStart":
        if data.get("_parse_error"):
            deny(DENY_BRIEF, 2)
        text = brief_text(data)
        if brief_denied(text):
            deny(DENY_BRIEF, 2)
        emit({"permission": "allow"})
        return

    if event == "subagentStop":
        parent = str(data.get("parent_conversation_id") or data.get("conversation_id") or cid)
        modified = data.get("modified_files") or data.get("files") or []
        if isinstance(modified, list):
            for item in modified:
                path = item if isinstance(item, str) else str(item.get("path") or item.get("file") or "")
                if path:
                    append_event(parent, "edit", path=path)
        emit({})
        return

    emit({})


if __name__ == "__main__":
    data = load_stdin()
    event = str(data.get("hook_event_name") or "")
    try:
        run(data)
    except SystemExit:
        raise
    except Exception:
        if event == "subagentStart":
            deny(DENY_BRIEF, 2)
        emit({})
