#!/usr/bin/env python3
from __future__ import annotations

import json
from collections import Counter
from pathlib import Path

LOG = Path.home() / ".cursor" / "hooks-state" / "obedience.jsonl"


def main() -> int:
    if not LOG.is_file():
        print("no obedience log yet:", LOG)
        return 0
    rows = []
    for line in LOG.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    if not rows:
        print("empty obedience log")
        return 0
    by_gate = Counter((r.get("gate"), r.get("permission")) for r in rows)
    tools = max(1, len(rows))
    print(f"decisions={len(rows)}")
    for (gate, perm), n in sorted(by_gate.items()):
        rate = 100.0 * n / tools
        print(f"{gate}\t{perm}\t{n}\t{rate:.1f}/100")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
