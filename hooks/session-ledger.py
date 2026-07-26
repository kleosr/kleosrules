#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import (  # noqa: E402
    emit,
    load_stdin,
    path_from,
    read_ledger,
    tool_input,
    write_ledger,
)

VERIFY = re.compile(
    r"(?i)(^|[;&|]\s*)("
    r"npm\s+test|pnpm\s+test|yarn\s+test|bun\s+test|"
    r"pytest|python\s+-m\s+pytest|"
    r"cargo\s+test|go\s+test|"
    r"tsc(\s|$)|ruff\s+check|eslint|"
    r"gradlew?\s+test|gradlew?\s+check|"
    r"make\s+test|cmake\s+--build|"
    r"hooks/_gauntlet\.py|hooks/_proof_evals\.py|hooks/_selftest\.py|"
    r"bash\s+.*TOOLCHAIN|npm\s+run\s+test"
    r")"
)
EDIT_TOOLS = {"Write", "StrReplace", "EditNotebook", "EditFile", "Delete", "ApplyPatch"}
ROOF = (
    "Master Mind roof: NO prose comments; ASK package installs; "
    "no remote publish without confirmation; verify before Done; "
    "never fight a deny."
)


def main() -> None:
    data = load_stdin()
    cid = str(data.get("conversation_id") or data.get("session_id") or "unknown")
    tool = str(data.get("tool_name") or "")
    led = read_ledger(cid)
    led["tools"] = int(led.get("tools") or 0) + 1
    if tool in EDIT_TOOLS or any(t in tool for t in ("Write", "StrReplace", "Edit", "Delete")):
        path = path_from(data)
        if path and not path.endswith((".md", ".mdc", ".txt", ".json")):
            led["edits"] = int(led.get("edits") or 0) + 1
    cmd = ""
    inp = tool_input(data)
    if isinstance(data.get("command"), str):
        cmd = data["command"]
    elif isinstance(inp.get("command"), str):
        cmd = inp["command"]
    if cmd and VERIFY.search(cmd):
        led["verifies"] = int(led.get("verifies") or 0) + 1
    write_ledger(cid, led)
    every = int(led.get("roof_every") or 12)
    out: dict = {}
    if every > 0 and led["tools"] % every == 0:
        out["additional_context"] = ROOF
    emit(out if out else {})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
