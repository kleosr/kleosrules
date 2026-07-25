#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
os_chdir = ROOT
probes = [
    ("hooks/deny-danger.sh", "git push origin main"),
    ("hooks/block-dangerous-git.sh", "git push origin main"),
    ("hooks/ask-gated-shell.sh", "git push origin main"),
    ("hooks/deny-danger.sh", "rm -rf payments"),
    ("hooks/deny-danger.sh", "rm -rf src"),
    ("hooks/deny-danger.sh", "rm -rf ./ledger"),
    ("hooks/deny-danger.sh", "rm -rf /"),
]


def run(script: str, cmd: str) -> str:
    p = subprocess.run(
        ["bash", str(ROOT / script)],
        input=json.dumps({"command": cmd}).encode(),
        capture_output=True,
        cwd=str(ROOT),
    )
    return p.stdout.decode().strip()


if __name__ == "__main__":
    for script, cmd in probes:
        out = run(script, cmd)
        print(f"{script} | {cmd} -> {out}")
