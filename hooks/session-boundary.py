#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import append_event, conversation_id, emit, freshness, load_stdin  # noqa: E402

ROOF = (
    "Master Mind roof: NO prose comments; ASK package installs; "
    "no remote publish without confirmation; verify before Done; "
    "never fight a deny."
)


def main() -> None:
    data = load_stdin()
    event = str(data.get("hook_event_name") or "")
    cid = conversation_id(data)

    if event == "sessionStart":
        fresh = freshness(cid)
        parts = [ROOF]
        unverified = fresh.get("unverified") or []
        if unverified:
            parts.append(
                "Session carry-over unverified paths: "
                + ", ".join(unverified)
                + ". Run verify-class commands before claiming Done."
            )
        emit({"additional_context": " ".join(parts)})
        return

    if event == "preCompact":
        append_event(cid, "compact")
        fresh = freshness(cid)
        dirty = fresh.get("unverified") or []
        out: dict = {}
        if dirty:
            out["user_message"] = (
                "Compaction pending — unverified edits on: "
                + ", ".join(dirty)
                + ". Re-verify after resume."
            )
        emit(out)
        return

    emit({})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
