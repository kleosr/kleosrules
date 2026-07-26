#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import (  # noqa: E402
    append_event,
    conversation_id,
    emit,
    freshness,
    load_stdin,
    path_from,
    read_events,
    tool_input,
    walk_strings,
)
from injection_lib import is_injection, notice  # noqa: E402

VERIFY = re.compile(
    r"(?i)(^|[;&|]\s*)("
    r"npm\s+test|pnpm\s+test|yarn\s+test|bun\s+test|"
    r"pytest|python\s+-m\s+pytest|"
    r"cargo\s+test|go\s+test|"
    r"tsc(\s|$)|ruff\s+check|eslint|"
    r"gradlew?\s+test|gradlew?\s+check|"
    r"make\s+test|cmake\s+--build|"
    r"hooks/_gauntlet\.py|hooks/_proof_evals\.py|hooks/_selftest\.py|hooks/_verify_hook_contracts\.py|"
    r"gate-shell\.py|gate-write\.py|"
    r"bash\s+.*TOOLCHAIN|npm\s+run\s+test"
    r")"
)
EDIT_TOOLS = {"Write", "StrReplace", "EditNotebook", "EditFile", "Delete", "ApplyPatch"}
ROOF = (
    "Master Mind roof: NO prose comments; ASK package installs; "
    "no remote publish without confirmation; verify before Done; "
    "never fight a deny."
)
ROOF_EVERY = 12


def tool_result_text(data: dict) -> str:
    parts: list[str] = []
    for key in ("result", "output", "content", "text", "stdout", "stderr"):
        v = data.get(key)
        if isinstance(v, str):
            parts.append(v)
        elif isinstance(v, dict):
            parts.extend(walk_strings(v))
    tr = data.get("tool_result")
    if isinstance(tr, dict):
        parts.extend(walk_strings(tr))
    elif isinstance(tr, str):
        parts.append(tr)
    return "\n".join(parts)


def mcp_output_text(data: dict) -> str:
    for key in ("mcp_tool_output", "tool_output", "output"):
        v = data.get(key)
        if isinstance(v, str):
            return v
        if isinstance(v, dict):
            return "\n".join(walk_strings(v))
    return ""


def main() -> None:
    data = load_stdin()
    cid = conversation_id(data)
    tool = str(data.get("tool_name") or "")
    append_event(cid, "tool", name=tool)

    if tool in EDIT_TOOLS or any(t in tool for t in ("Write", "StrReplace", "Edit", "Delete")):
        path = path_from(data)
        if path and not path.endswith((".md", ".mdc", ".txt", ".json")):
            append_event(cid, "edit", path=path)

    inp = tool_input(data)
    cmd = ""
    if isinstance(data.get("command"), str):
        cmd = data["command"]
    elif isinstance(inp.get("command"), str):
        cmd = inp["command"]
    if cmd and VERIFY.search(cmd):
        append_event(cid, "verify", cmd=cmd[:200])

    out: dict = {}
    contexts: list[str] = []

    inj_text = tool_result_text(data)
    if inj_text and is_injection(inj_text):
        contexts.append(notice(inj_text))

    mcp_text = mcp_output_text(data)
    if mcp_text and is_injection(mcp_text):
        contexts.append(notice(mcp_text))
        out["updated_mcp_tool_output"] = json.dumps(
            {"warning": notice(mcp_text), "filtered": True}
        )

    updated = tool_input(data)
    for ev in reversed(read_events(cid)[-8:]):
        if ev.get("normalized"):
            contexts.append("Normalize write stripped prose comments from payload.")
            break

    fresh = freshness(cid)
    if fresh.get("tools", 0) % ROOF_EVERY == 0:
        contexts.append(ROOF)

    if contexts:
        out["additional_context"] = " ".join(contexts)

    emit(out if out else {})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
