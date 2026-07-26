#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import ask, deny, emit, load_stdin, log_decision, mcp_blobs  # noqa: E402

SECRET_RE = re.compile(
    r"(?i)(sk_live_[A-Za-z0-9]{16,}|sk_test_[A-Za-z0-9]{16,}|"
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|"
    r"xox[baprs]-[A-Za-z0-9-]{20,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----)"
)
DANGER = re.compile(
    r"(?i)(drop[_ ]?table|delete[_ ]?database|destroy|force[_ ]?push|"
    r"rm[_ ]?rf|truncate|purchase|deploy|delete_vm|terminate)"
)
MSG_SECRET = "Blocked secret-like material on MCP tool input."
MSG_ASK = "MCP tool looks mutating/destructive. Confirm exact tool and args."


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        deny("gate-mcp parse error", 2)
    event = str(data.get("hook_event_name") or "beforeMCPExecution")
    tool = str(data.get("tool_name") or data.get("name") or "")
    blobs = mcp_blobs(data)
    text = "\n".join(blobs)
    if text and SECRET_RE.search(text) and "EXAMPLE_SECRET" not in text:
        log_decision("gate-mcp", event, "deny", tool)
        deny(MSG_SECRET, 2)
    joined = tool + "\n" + text
    if DANGER.search(joined):
        log_decision("gate-mcp", event, "ask", tool)
        ask(MSG_ASK)
    emit({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        deny("gate-mcp failed closed", 2)
