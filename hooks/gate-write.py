#!/usr/bin/env python3
from __future__ import annotations

import copy
import os
import re
import sys
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from hookio import (  # noqa: E402
    append_event,
    content_blobs,
    conversation_id,
    deny,
    emit,
    fingerprint,
    load_stdin,
    log_decision,
    path_from,
    prior_deny_count,
    record_deny_fp,
    tool_input,
    walk_strings,
)
from prose_comment_lib import deny_payload, path_is_code, strip_prose, violates  # noqa: E402

SECRET_RE = re.compile(
    r"(?i)(sk_live_[A-Za-z0-9]{16,}|sk_test_[A-Za-z0-9]{16,}|"
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|"
    r"xox[baprs]-[A-Za-z0-9-]{20,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----)"
)
MSG = "Blocked secret-like material (lab gate)."
REPEAT_MSG = (
    "Repeat identical blocked write (freeze loop). Rewrite to an allowed surface or stop — "
    "do not retry the same blocked payload."
)
CONTENT_KEYS = (
    "contents",
    "content",
    "new_string",
    "old_string",
    "cell_content",
    "new_source",
)


def load_vern():
    import importlib.util

    path = HERE / "deny-vernacular-drift.py"
    spec = importlib.util.spec_from_file_location("deny_vernacular_drift", path)
    mod = importlib.util.module_from_spec(spec)
    assert spec and spec.loader
    spec.loader.exec_module(mod)
    return mod


def normalize_enabled() -> bool:
    return os.environ.get("KLEOS_NORMALIZE", "1") not in ("0", "false", "False", "no")


def strip_in_obj(obj: Any, path: str) -> tuple[Any, bool] | None:
    if isinstance(obj, str):
        stripped = strip_prose(path, obj)
        if stripped is None:
            return None
        return stripped, stripped != obj
    if isinstance(obj, dict):
        out: dict = {}
        changed = False
        for key, val in obj.items():
            if key in CONTENT_KEYS and isinstance(val, str):
                stripped = strip_prose(path, val)
                if stripped is None:
                    return None
                out[key] = stripped
                if stripped != val:
                    changed = True
            elif key == "edits" and isinstance(val, list):
                edits_out = []
                for ed in val:
                    if isinstance(ed, dict):
                        res = strip_in_obj(ed, path)
                        if res is None:
                            return None
                        new_ed, ch = res
                        edits_out.append(new_ed)
                        changed = changed or ch
                    else:
                        edits_out.append(ed)
                out[key] = edits_out
            else:
                out[key] = val
        return out, changed
    return obj, False


def apply_strip(inp: dict, path: str) -> dict | None:
    res = strip_in_obj(inp, path)
    if res is None:
        return None
    updated, _ = res
    return updated if isinstance(updated, dict) else None


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        emit(deny_payload(), 2)
    event = str(data.get("hook_event_name") or "preToolUse")
    cid = conversation_id(data)
    inp = tool_input(data)
    blobs = walk_strings(inp) + walk_strings(data)
    text = "\n".join(blobs)
    if text and SECRET_RE.search(text) and "EXAMPLE_SECRET" not in text:
        log_decision("gate-write", event, "deny", "secret")
        if event == "beforeSubmitPrompt":
            emit({"continue": False, "user_message": MSG}, 2)
        deny(MSG, 2)

    path = path_from(data)
    body = "\n".join(content_blobs(inp) or content_blobs(data))

    if violates(path, body):
        if normalize_enabled() and path:
            updated_inp = apply_strip(inp, path)
            if updated_inp is not None:
                new_body = "\n".join(content_blobs(updated_inp))
                if not violates(path, new_body):
                    append_event(cid, "edit", path=path, normalized=True)
                    emit({"permission": "allow", "updated_input": updated_inp})
        fp = fingerprint({"path": path, "body": body, "event": event})
        repeat = prior_deny_count(cid, fp) >= 1
        record_deny_fp(cid, fp)
        if repeat:
            append_event(cid, "deny_repeat", fp=fp, path=path)
            log_decision("gate-write", event, "deny", "repeat")
            emit(
                {
                    "permission": "deny",
                    "user_message": REPEAT_MSG,
                    "agent_message": REPEAT_MSG,
                },
                2,
            )
        log_decision("gate-write", event, "deny", path)
        emit(deny_payload(), 2)

    if path and path_is_code(path):
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

    if path and body:
        append_event(cid, "edit", path=path)

    emit({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit(deny_payload(), 2)
