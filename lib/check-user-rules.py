#!/usr/bin/env python3
"""Best-effort: detect if USER-RULES paste text is present in Cursor local state.

Exit 0 if found, 1 if not. Never writes. Cloud-only User Rules may not appear
here even when active in the UI — treat 1 as a warning, not hard proof.
"""
from __future__ import annotations

import sqlite3
import sys
from pathlib import Path

SSOT = Path("/home/kleosr/Documentos/rules")
PASTE = SSOT / "USER-RULES.paste.txt"
DB = Path.home() / ".config/Cursor/User/globalStorage/state.vscdb"

MARKERS = (
    "lazy senior engineer",
    "Restraint over sophistication",
    "PRECEDENCE: Team → Project → User",
    "Documentos/rules/agent.mdc",
)


def main() -> int:
    if not PASTE.is_file():
        print("no USER-RULES.paste.txt", file=sys.stderr)
        return 1
    text = PASTE.read_text(encoding="utf-8", errors="replace")
    needles = [m for m in MARKERS if m in text]
    if not needles:
        needles = ["lazy senior engineer"]

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
        if "lazy senior engineer" in val and "Documentos/rules" in val:
            return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
