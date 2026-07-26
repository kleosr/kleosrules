#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from hookio import deny, emit, load_stdin, log_decision, path_from, tool_input, walk_strings  # noqa: E402
from prose_comment_lib import deny_payload, violates  # noqa: E402

SECRET_RE = re.compile(
    r"(?i)(sk_live_[A-Za-z0-9]{16,}|sk_test_[A-Za-z0-9]{16,}|"
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|"
    r"xox[baprs]-[A-Za-z0-9-]{20,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----)"
)
MSG = "Blocked secret-like material (lab gate)."


def load_vern():
    path = HERE / "deny-vernacular-drift.py"
    spec = importlib.util.spec_from_file_location("deny_vernacular_drift", path)
    mod = importlib.util.module_from_spec(spec)
    assert spec and spec.loader
    spec.loader.exec_module(mod)
    return mod


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        emit(deny_payload(), 2)
    event = str(data.get("hook_event_name") or "preToolUse")
    inp = tool_input(data)
    blobs = walk_strings(inp) + walk_strings(data)
    text = "\n".join(blobs)
    if text and SECRET_RE.search(text) and "EXAMPLE_SECRET" not in text:
        log_decision("gate-write", event, "deny", "secret")
        if event == "beforeSubmitPrompt":
            emit({"continue": False, "user_message": MSG}, 2)
        deny(MSG, 2)

    path = path_from(data)
    body = "\n".join(walk_strings(inp))
    if violates(path, body):
        log_decision("gate-write", event, "deny", path)
        emit(deny_payload(), 2)

    if path:
        vern = load_vern()
        p = Path(path)
        contract = vern.find_contract(p if p.exists() else p.parent)
        if contract is not None:
            fields = vern.parse_fields(contract.read_text(encoding="utf-8", errors="replace"))
            if fields:
                if not vern.file_name_ok(p.name, fields):
                    log_decision("gate-write", event, "deny", "vernacular-name")
                    deny(f"Blocked vernacular file-name drift: {p.name}", 2)
                if body:
                    ok, reason = vern.naming_ok(body, fields)
                    if not ok:
                        log_decision("gate-write", event, "deny", "vernacular-name")
                        deny(f"Blocked vernacular naming drift: {reason}", 2)

    emit({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit(deny_payload(), 2)
