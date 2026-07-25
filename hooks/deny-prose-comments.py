#!/usr/bin/env python3
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from prose_comment_lib import deny_payload, violates  # noqa: E402


def out(d):
    print(json.dumps(d))
    raise SystemExit(0)


try:
    try:
        data = json.load(sys.stdin)
    except Exception:
        out(deny_payload())

    inp = data.get("input") or data.get("tool_input") or {}
    if not isinstance(inp, dict):
        inp = {}

    path = str(inp.get("path") or data.get("path") or data.get("file_path") or "")
    chunks = []
    for k in ("contents", "new_string", "content", "cell_content", "new_source"):
        v = inp.get(k)
        if isinstance(v, str):
            chunks.append(v)
        v2 = data.get(k)
        if isinstance(v2, str):
            chunks.append(v2)

    text = "\n".join(chunks)
    if violates(path, text):
        out(deny_payload())
    out({"permission": "allow"})
except Exception:
    out(deny_payload())
