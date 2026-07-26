#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import (  # noqa: E402
    append_event,
    conversation_id,
    emit,
    fingerprint,
    load_stdin,
    prior_deny_count,
    record_deny_fp,
)


def main() -> None:
    data = load_stdin()
    cid = conversation_id(data)
    reason = str(data.get("reason") or data.get("error") or data.get("failure_reason") or "")
    perm = str(data.get("permission") or data.get("result") or "")
    denied = perm == "permission_denied" or "permission_denied" in reason.lower() or "deny" in reason.lower()
    if not denied:
        emit({})
    fp_payload = {
        "tool": data.get("tool_name"),
        "tool_use_id": data.get("tool_use_id"),
        "reason": reason[:200],
    }
    fp = fingerprint(fp_payload)
    if prior_deny_count(cid, fp) >= 1:
        append_event(cid, "deny_repeat", fp=fp, tool=str(data.get("tool_name") or ""))
    record_deny_fp(cid, fp)
    emit({})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
