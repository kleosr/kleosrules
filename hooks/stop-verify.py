#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import emit, load_stdin, read_ledger  # noqa: E402

FOLLOW = (
    "Session edited application code without a verify-class command. "
    "Run the house gauntlet (TOOLCHAIN / tests) and cite evidence, "
    "or name residual risk and ask accept-no-gauntlet-risk. "
    "Do not claim Done without verification evidence."
)


def main() -> None:
    data = load_stdin()
    status = str(data.get("status") or "")
    if status and status != "completed":
        emit({})
    cid = str(data.get("conversation_id") or data.get("session_id") or "unknown")
    led = read_ledger(cid)
    edits = int(led.get("edits") or 0)
    verifies = int(led.get("verifies") or 0)
    if edits > 0 and verifies == 0:
        emit({"followup_message": FOLLOW})
    emit({})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({})
