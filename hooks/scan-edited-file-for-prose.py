#!/usr/bin/env python3
"""afterFileEdit: if edited CODE_EXT file on disk has prose comments, deny/follow-up strip."""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from prose_comment_lib import (  # noqa: E402
    DENY_AGENT,
    DENY_USER,
    path_is_code,
    text_has_prose,
)


def out(d):
    print(json.dumps(d))
    raise SystemExit(0)


try:
    data = json.load(sys.stdin)
except Exception:
    out({})

inp = data.get("input") or data.get("tool_input") or {}
if not isinstance(inp, dict):
    inp = {}

path = str(
    inp.get("path")
    or inp.get("file_path")
    or data.get("path")
    or data.get("file_path")
    or data.get("filePath")
    or ""
)

if not path or not path_is_code(path):
    out({})

try:
    text = Path(path).read_text(encoding="utf-8", errors="replace")
except OSError:
    out({})

if text_has_prose(text, path):
    out({
        "permission": "deny",
        "user_message": DENY_USER,
        "agent_message": (
            DENY_AGENT
            + f" File still has prose comments: {path}. Strip them with Write/StrReplace now."
        ),
        "followup_message": (
            f"Prose comments landed in {path}. Strip all // # /* */ prose on that path now "
            "(machine directives only if required for green build)."
        ),
    })
out({})
