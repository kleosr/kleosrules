#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import emit, freshness, load_stdin  # noqa: E402

FOLLOW = (
    "Session has unverified edits. Run the house gauntlet (TOOLCHAIN / tests) "
    "and cite evidence, or name residual risk and ask accept-no-gauntlet-risk. "
    "Do not claim Done without verification evidence."
)
LOOP_MSG = (
    "Freeze loop detected (repeat deny fingerprints). Stop retrying the same blocked "
    "write; rewrite to an allowed surface or ask the user."
)


def main() -> None:
    data = load_stdin()
    status = str(data.get("status") or "")
    if status and status != "completed":
        emit({})
    cid = str(data.get("conversation_id") or data.get("session_id") or "unknown")
    fresh = freshness(cid)
    unverified = fresh.get("unverified") or []
    loops = int(fresh.get("loops") or 0)
    if loops >= 2:
        emit({"followup_message": LOOP_MSG + " Unverified: " + ", ".join(unverified)})
    if unverified:
        emit(
            {
                "followup_message": FOLLOW
                + " Unverified paths: "
                + ", ".join(unverified)
            }
        )
    emit({})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
