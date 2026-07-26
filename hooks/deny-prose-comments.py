#!/usr/bin/env python3
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import emit, load_stdin, log_decision, path_from, tool_input, walk_strings  # noqa: E402
from prose_comment_lib import deny_payload, violates  # noqa: E402


try:
    data = load_stdin()
    if data.get("_parse_error"):
        emit(deny_payload(), 2)
    inp = tool_input(data)
    path = path_from(data) or str(inp.get("path") or data.get("path") or data.get("file_path") or "")
    chunks = walk_strings(inp) + walk_strings(
        {k: data[k] for k in data if k not in ("tool_input", "input")}
    )
    text = "\n".join(chunks)
    if violates(path, text):
        log_decision("deny-prose-comments", str(data.get("hook_event_name") or "preToolUse"), "deny", path)
        emit(deny_payload(), 2)
    emit({"permission": "allow"})
except Exception:
    emit(deny_payload(), 2)
