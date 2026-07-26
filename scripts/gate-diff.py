#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "hooks"))
from prose_comment_lib import violates  # noqa: E402

EXEMPT = {"injection_lib.py", "_gauntlet.py"}
CODE_GLOB = ("*.py", "*.sh")


def scan_tree() -> list[str]:
    fails: list[str] = []
    for pattern in CODE_GLOB:
        for path in ROOT.rglob(pattern):
            if "node_modules" in path.parts or ".git" in path.parts:
                continue
            if path.name in EXEMPT:
                continue
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            if violates(str(path), text):
                fails.append(str(path.relative_to(ROOT)))
    return fails


def main() -> int:
    fails = scan_tree()
    if fails:
        for f in fails:
            print(f"[FAIL] prose comment: {f}")
        print(f"[FAIL] {len(fails)} file(s) with prose comments")
        return 1
    print("GATE_DIFF_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
