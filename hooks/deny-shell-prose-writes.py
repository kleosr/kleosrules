#!/usr/bin/env python3
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import command_from, emit, load_stdin, log_decision  # noqa: E402
from prose_comment_lib import ask_opaque_payload, deny_payload, shell_write_class  # noqa: E402


try:
    data = load_stdin()
    if data.get("_parse_error"):
        emit(deny_payload(), 2)
    cmd = command_from(data)
    kind = shell_write_class(cmd)
    if kind == "prose":
        log_decision("deny-shell-prose-writes", "beforeShellExecution", "deny", cmd[:120])
        emit(deny_payload(), 2)
    if kind == "opaque":
        log_decision("deny-shell-prose-writes", "beforeShellExecution", "ask", cmd[:120])
        emit(ask_opaque_payload(), 0)
    emit({"permission": "allow"})
except Exception:
    emit(deny_payload(), 2)
