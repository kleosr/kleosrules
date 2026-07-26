#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
PACK = HERE.parent


def run_py(script: str, payload: dict) -> dict:
    p = subprocess.run(
        [sys.executable, str(HERE / script)],
        input=json.dumps(payload).encode(),
        capture_output=True,
        cwd=str(PACK),
    )
    out = p.stdout.decode().strip()
    return json.loads(out) if out else {}


def run_sh(script: str, payload: dict) -> dict:
    p = subprocess.run(
        ["bash", str(HERE / script)],
        input=json.dumps(payload).encode(),
        capture_output=True,
        cwd=str(PACK),
    )
    out = p.stdout.decode().strip()
    return json.loads(out) if out else {}


def main() -> int:
    slash = "/" + "/"
    assert run_py(
        "deny-prose-comments.py",
        {"input": {"path": "a.ts", "contents": f"x=1\n{slash} bad\n"}},
    )["permission"] == "deny"

    assert run_py(
        "deny-prose-comments.py",
        {"tool_input": {"file_path": "a.ts", "edits": [{"new_string": f"{slash} why"}]}},
    )["permission"] == "deny"

    assert run_py(
        "deny-prose-comments.py",
        {"input": {"path": "a.rb", "contents": "# prose\n"}},
    )["permission"] == "deny"

    assert run_py(
        "deny-shell-prose-writes.py",
        {"command": "echo Ly8gd2h5 | base64 -d >> src/a.ts"},
    )["permission"] == "deny"

    assert run_py(
        "deny-shell-prose-writes.py",
        {"command": "git apply p.patch"},
    )["permission"] == "ask"

    assert run_py(
        "gate-read.py",
        {"hook_event_name": "beforeReadFile", "file_path": ".env"},
    )["permission"] == "deny"

    assert run_py(
        "gate-mcp.py",
        {
            "hook_event_name": "beforeMCPExecution",
            "tool_name": "postgres_drop_table",
            "tool_input": {"table": "t"},
        },
    )["permission"] == "ask"

    assert run_py(
        "gate-delete.py",
        {"tool_name": "Delete", "tool_input": {"path": "payments", "recursive": True}},
    )["permission"] == "deny"

    assert run_py(
        "gate-write.py",
        {"hook_event_name": "preToolUse", "tool_input": {"path": "a.ts", "contents": f"x=1\n{slash} why\n"}},
    )["permission"] == "deny"

    _ = "session-ledger.py stop-verify.py gate-write.py gate-read.py gate-mcp.py gate-delete.py"

    assert run_py(
        "deny-prose-comments.py",
        {"input": {"path": "a.ts", "contents": 'const u = "http://example.com";\n'}},
    )["permission"] == "allow"

    assert run_py(
        "deny-shell-prose-writes.py",
        {"command": f'printf "%s\\n" "{slash} why" >> src/foo.ts'},
    )["permission"] == "deny"

    with tempfile.TemporaryDirectory() as td:
        root = Path(td)
        vdir = root / ".cursor" / "rules"
        vdir.mkdir(parents=True)
        (vdir / "vernacular.mdc").write_text(
            "file_name_pattern: domain.kind.ext\n"
            "allowed_kinds: usecase, service\n"
            "class_pattern: PascalCase\n"
            "function_pattern: verbObject\n",
            encoding="utf-8",
        )
        bad = root / "src" / "BadName.ts"
        bad.parent.mkdir(parents=True)
        r = run_py(
            "deny-vernacular-drift.py",
            {"input": {"path": str(bad), "contents": "export function createUser() {}\n"}},
        )
        assert r["permission"] == "deny", r

        good = root / "src" / "user.create.usecase.ts"
        r2 = run_py(
            "deny-vernacular-drift.py",
            {"input": {"path": str(good), "contents": "export function createUser() {}\n"}},
        )
        assert r2["permission"] == "allow", r2

        r3 = run_py(
            "deny-vernacular-drift.py",
            {
                "input": {
                    "path": str(good),
                    "contents": "export function Bad_Name() {}\n",
                }
            },
        )
        assert r3["permission"] == "deny", r3

    force = run_sh(
        "block-dangerous-git.sh",
        {"command": "git push origin main --force"},
    )
    assert force.get("permission") == "deny", force

    push = run_sh("ask-gated-shell.sh", {"command": "git push origin main"})
    assert push.get("permission") == "ask", push

    tree_rm = run_sh("ask-gated-shell.sh", {"command": "rm -rf payments"})
    assert tree_rm.get("permission") == "ask", tree_rm

    root_rm = run_sh("deny-danger.sh", {"command": "rm -rf /"})
    assert root_rm.get("permission") == "deny", root_rm

    npx = run_sh("ask-gated-shell.sh", {"command": "npx cowsay hi"})
    assert npx.get("permission") == "ask", npx

    inst = run_sh("ask-gated-shell.sh", {"command": "npm install lodash"})
    assert inst.get("permission") == "ask", inst

    ci = run_sh("ask-gated-shell.sh", {"command": "npm ci"})
    assert ci.get("permission") == "ask", ci

    rel = run_sh("ask-gated-shell.sh", {"command": "gh release create v1"})
    assert rel.get("permission") == "ask", rel

    find_del = run_sh("ask-gated-shell.sh", {"command": "find . -delete"})
    assert find_del.get("permission") == "ask", find_del

    prompt_secret = run_py(
        "block-secrets.py",
        {"hook_event_name": "beforeSubmitPrompt", "prompt": "ship ghp_" + "A" * 24},
    )
    assert prompt_secret.get("continue") is False, prompt_secret

    write_secret = run_py(
        "block-secrets.py",
        {
            "hook_event_name": "preToolUse",
            "tool_name": "Write",
            "tool_input": {
                "path": "a.ts",
                "contents": "-----BEGIN " + "RSA PRIVATE KEY-----\n",
            },
        },
    )
    assert write_secret.get("permission") == "deny", write_secret

    print("ALL_PROOF_EVALS_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
