#!/usr/bin/env python3
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from prose_comment_lib import deny_payload, shell_prose_write  # noqa: E402


def out(d):
    print(json.dumps(d))
    raise SystemExit(0)


try:
    try:
        data = json.load(sys.stdin)
    except Exception:
        out(deny_payload())

    cmd = data.get("command") or data.get("cmd") or ""
    if isinstance(data.get("input"), dict):
        cmd = cmd or data["input"].get("command") or data["input"].get("cmd") or ""
    cmd = str(cmd)

    if shell_prose_write(cmd):
        out(deny_payload())
    out({"permission": "allow"})
except Exception:
    out(deny_payload())
