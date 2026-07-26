#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
PACK = HERE.parent
SLASH = "/" + "/"


def run(script: str, payload: dict, env: dict | None = None) -> tuple[int, dict]:
    e = os.environ.copy()
    if env:
        e.update(env)
    exe = ["bash", str(HERE / script)] if script.endswith(".sh") else [sys.executable, str(HERE / script)]
    p = subprocess.run(
        exe,
        input=json.dumps(payload).encode(),
        capture_output=True,
        cwd=str(PACK),
        env=e,
    )
    out = p.stdout.decode().strip()
    try:
        obj = json.loads(out) if out else {}
    except json.JSONDecodeError:
        obj = {"_raw": out}
    return p.returncode, obj


def must(cond: bool, msg: str) -> None:
    if not cond:
        raise AssertionError(msg)


def stub_bin(*names: str) -> Path:
    td = Path(tempfile.mkdtemp(prefix="gate_stub_"))
    for n in names:
        p = td / n
        p.write_text("#!/bin/sh\nexit 127\n", encoding="utf-8")
        p.chmod(0o755)
    return td


def main() -> int:
    fails: list[str] = []

    def check(name: str, fn) -> None:
        try:
            fn()
            print(f"[ok] {name}")
        except Exception as ex:
            fails.append(f"{name}: {ex}")
            print(f"[FAIL] {name}: {ex}")

    def p0_jq_dead() -> None:
        stub = stub_bin("jq")
        env = {"PATH": f"{stub}:/usr/bin:/bin"}
        code, obj = run("ask-gated-shell.sh", {"command": "npm ci"}, env)
        must(code == 0 and obj.get("permission") == "ask", f"with real python ask got {code} {obj}")
        stub2 = stub_bin("python3")
        env2 = {"PATH": f"{stub2}:/bin"}
        code, obj = run("deny-danger.sh", {"command": "npm publish"}, env2)
        must(code == 2 and obj.get("permission") == "deny", f"no python deny got {code} {obj}")
        code, obj = run("block-dangerous-git.sh", {"command": "git push --force origin main"}, env2)
        must(obj.get("permission") == "deny", f"missing python must deny force-push, got {obj}")

    def p3_shapes() -> None:
        code, obj = run(
            "deny-prose-comments.py",
            {"tool_input": {"file_path": "src/a.ts", "edits": [{"new_string": f"{SLASH} why"}]}},
        )
        must(obj.get("permission") == "deny", f"edits array {obj}")
        code, obj = run(
            "deny-prose-comments.py",
            {"tool_input": {"path": "src/a.mts", "contents": f"{SLASH} why\n"}},
        )
        must(obj.get("permission") == "deny", f"mts {obj}")
        code, obj = run(
            "deny-prose-comments.py",
            {"tool_input": {"path": "a.rb", "contents": "# prose\n"}},
        )
        must(obj.get("permission") == "deny", f"rb {obj}")
        code, obj = run(
            "deny-prose-comments.py",
            {"tool_input": {"path": "a.lua", "contents": "-- prose\n"}},
        )
        must(obj.get("permission") == "deny", f"lua {obj}")

    def p4_shell() -> None:
        code, obj = run(
            "deny-shell-prose-writes.py",
            {"command": "echo Ly8gd2h5 | base64 -d >> src/a.ts"},
        )
        must(obj.get("permission") == "deny", f"b64 {obj}")
        code, obj = run(
            "deny-shell-prose-writes.py",
            {"command": "sed -i '1i // why' src/a.ts"},
        )
        must(obj.get("permission") == "ask", f"sed opaque {obj}")
        code, obj = run(
            "deny-shell-prose-writes.py",
            {"command": "git apply p.patch"},
        )
        must(obj.get("permission") == "ask", f"git apply {obj}")

    def p5_surfaces() -> None:
        code, obj = run("gate-read.py", {"file_path": ".env", "hook_event_name": "beforeReadFile"})
        must(obj.get("permission") == "deny", f"read env {obj}")
        code, obj = run("gate-read.py", {"file_path": ".env.example", "hook_event_name": "beforeReadFile"})
        must(obj.get("permission") == "allow", f"env example {obj}")
        code, obj = run(
            "gate-mcp.py",
            {
                "hook_event_name": "beforeMCPExecution",
                "tool_name": "postgres_drop_table",
                "tool_input": {"table": "users"},
            },
        )
        must(obj.get("permission") == "ask", f"mcp danger {obj}")
        code, obj = run(
            "gate-delete.py",
            {"tool_name": "Delete", "tool_input": {"path": "payments", "recursive": True}},
        )
        must(obj.get("permission") == "deny", f"delete tree {obj}")

    def p2_gate_write() -> None:
        code, obj = run(
            "gate-write.py",
            {
                "hook_event_name": "preToolUse",
                "tool_input": {"path": "a.ts", "contents": f"x=1\n{SLASH} why\n"},
            },
        )
        must(obj.get("permission") == "deny", f"gate-write prose {obj}")

    def meters() -> None:
        for name in ("_selftest.py", "_proof_evals.py", "_verify_hook_contracts.py"):
            p = subprocess.run([sys.executable, str(HERE / name)], cwd=str(PACK), capture_output=True, text=True)
            must(p.returncode == 0, f"{name} exit {p.returncode}: {(p.stdout+p.stderr)[-400:]}")

    check("P0 parser/missing-python", p0_jq_dead)
    check("P2 gate-write", p2_gate_write)
    check("P3 shapes/ext", p3_shapes)
    check("P4 shell channels", p4_shell)
    check("P5 read/mcp/delete", p5_surfaces)
    check("meters", meters)

    if fails:
        print(f"[FAIL] {len(fails)} gauntlet case(s)")
        return 1
    print("ALL_GAUNTLET_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
