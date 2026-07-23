#!/usr/bin/env python3
"""Best-effort: detect if Option C User Rules paste is present in Cursor local state.

Exit 0 if found, 1 if not. Cloud-only User Rules may not appear here.
"""
from __future__ import annotations

import sqlite3
import sys
from pathlib import Path

SSOT = Path(__file__).resolve().parents[1]
PASTE = SSOT / "user-rules" / "USER-RULES.paste.txt"
DB = Path.home() / ".config/Cursor/User/globalStorage/state.vscdb"

MARKERS = (
    "Think / Fix / Check",
    "NATIVE LEAN",
    "OBEDIENCE STACK",
    "NO COMMENTS",
    "RISK CONTRACT",
)


def main() -> int:
    if not PASTE.is_file():
        print("no USER-RULES.paste.txt", file=sys.stderr)
        return 1
    text = PASTE.read_text(encoding="utf-8", errors="replace")
    needles = [m for m in MARKERS if m in text]
    if len(needles) < 2:
        needles = list(MARKERS[:3])

    if not DB.is_file():
        print("no state.vscdb", file=sys.stderr)
        return 1

    con = sqlite3.connect(f"file:{DB}?mode=ro", uri=True)
    try:
        rows = con.execute("SELECT value FROM ItemTable").fetchall()
    finally:
        con.close()

    for (val,) in rows:
        if val is None:
            continue
        if isinstance(val, bytes):
            try:
                val = val.decode("utf-8", "replace")
            except Exception:
                continue
        if not isinstance(val, str):
            continue
        hits = sum(1 for n in needles if n in val)
        if hits >= 2:
            return 0
        if "NATIVE LEAN" in val and "V9.2" in val:
            return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
