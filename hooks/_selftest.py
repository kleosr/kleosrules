#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent


def run(script: str, payload: dict) -> dict:
    p = subprocess.run(
        [sys.executable, str(HERE / script)],
        input=json.dumps(payload).encode(),
        capture_output=True,
    )
    out = p.stdout.decode().strip()
    if not out:
        return {}
    return json.loads(out)


def main() -> int:
    slash_bad = "x=1\n" + "/" + "/ bad\n"
    url_ok = 'const u = "http:' + "/" + '/example.com";\n'
    directive_ok = "/" + "/ @ts-expect-error intentional\nx=1\n"

    assert run("deny-prose-comments.py", {"input": {"path": "a.ts", "contents": slash_bad}})["permission"] == "deny"
    assert run("deny-prose-comments.py", {"tool_input": {"file_path": "a.ts", "edits": [{"new_string": "/" + "/ bad"}]}})["permission"] == "deny"
    assert run("deny-prose-comments.py", {"input": {"path": "", "contents": slash_bad}})["permission"] == "deny"
    assert run("deny-prose-comments.py", {"input": {"path": "a.vue", "contents": slash_bad}})["permission"] == "deny"
    assert run("deny-prose-comments.py", {"input": {"path": "a.ts", "contents": "x=1\n"}})["permission"] == "allow"
    assert run("deny-prose-comments.py", {"input": {"path": "a.ts", "contents": url_ok}})["permission"] == "allow"
    assert run("deny-prose-comments.py", {"input": {"path": "a.ts", "contents": directive_ok}})["permission"] == "allow"

    pstar = 'printf "%s\\n" "' + "/" + '/ why" >> src/foo.ts'
    assert run("deny-shell-prose-writes.py", {"command": pstar})["permission"] == "deny"
    assert run("deny-shell-prose-writes.py", {"command": "ls src"})["permission"] == "allow"
    assert run("deny-shell-prose-writes.py", {"command": "echo ok >> src/foo.ts"})["permission"] == "allow"

    with tempfile.NamedTemporaryFile(suffix=".ts", mode="w", delete=False, encoding="utf-8") as f:
        f.write("const x = 1\n" + "/" + "/ bad\n")
        bad_path = f.name
    r = run("scan-edited-file-for-prose.py", {"path": bad_path})
    assert r.get("permission") == "deny" or "agent_message" in r
    Path(bad_path).write_text("const x = 1\n", encoding="utf-8")
    r2 = run("scan-edited-file-for-prose.py", {"path": bad_path})
    assert r2 == {} or r2.get("permission") != "deny"
    Path(bad_path).unlink(missing_ok=True)

    print("ALL_HOOK_TESTS_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
