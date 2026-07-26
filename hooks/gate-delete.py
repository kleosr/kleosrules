#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import deny, emit, load_stdin, log_decision, tool_input, walk_strings  # noqa: E402

ROUTE = (
    "Native Delete cannot ask (preToolUse ask is unenforced). "
    "State the exact path list, get user assent, then delete via Shell "
    "so beforeShellExecution can ask."
)


def is_treeish(data: dict) -> bool:
    inp = tool_input(data)
    if inp.get("recursive") in (True, "true", "True", 1, "1"):
        return True
    paths: list[str] = []
    for k in ("path", "paths", "files", "targets", "file_path"):
        v = inp.get(k) or data.get(k)
        if isinstance(v, str):
            paths.append(v)
        elif isinstance(v, list):
            paths.extend(str(x) for x in v)
    blobs = walk_strings(inp)
    paths.extend(blobs)
    joined = " ".join(paths)
    if len(paths) > 1:
        return True
    for p in paths:
        s = str(p).rstrip("/")
        base = s.rsplit("/", 1)[-1]
        if not base or "." not in base:
            if s.endswith(("src", "lib", "app", "packages", "payments", "ledger")):
                return True
            if "/" in s and "." not in base:
                return True
    if "*" in joined or joined.strip() in (".", "/", "~"):
        return True
    return False


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        deny("gate-delete parse error", 2)
    event = str(data.get("hook_event_name") or "preToolUse")
    if is_treeish(data):
        log_decision("gate-delete", event, "deny", "tree")
        deny(
            "Blocked tree/mass Delete via native tool. " + ROUTE,
            2,
        )
    emit({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        deny("gate-delete failed closed", 2)
