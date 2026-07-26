#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
PACK = HERE.parent
HOOKS_JSON = HERE / "hooks.json"

KNOWN_EVENTS = {
    "sessionStart", "sessionEnd", "preToolUse", "postToolUse",
    "postToolUseFailure", "subagentStart", "subagentStop",
    "beforeShellExecution", "afterShellExecution", "beforeMCPExecution",
    "afterMCPExecution", "beforeReadFile", "afterFileEdit",
    "beforeSubmitPrompt", "preCompact", "stop", "afterAgentResponse",
    "afterAgentThought", "beforeTabFileRead", "afterTabFileEdit",
    "workspaceOpen",
}

OUTPUT_FIELDS = {
    "beforeSubmitPrompt": {"continue", "user_message"},
    "preToolUse": {"permission", "user_message", "agent_message", "updated_input"},
    "beforeShellExecution": {"permission", "user_message", "agent_message", "continue"},
    "beforeMCPExecution": {"permission", "user_message", "agent_message", "continue"},
    "beforeReadFile": {"permission", "user_message"},
    "beforeTabFileRead": {"permission"},
    "postToolUse": {"additional_context", "followup_message"},
    "stop": {"followup_message"},
    "afterFileEdit": set(),
    "afterTabFileEdit": set(),
}

BLOCK_FIELD = {
    "beforeSubmitPrompt": "continue",
    "preToolUse": "permission",
    "beforeShellExecution": "permission",
    "beforeMCPExecution": "permission",
    "beforeReadFile": "permission",
    "beforeTabFileRead": "permission",
}

ENFORCED_PERMISSIONS = {
    "preToolUse": {"allow", "deny"},
    "beforeShellExecution": {"allow", "deny", "ask"},
    "beforeMCPExecution": {"allow", "deny", "ask"},
    "beforeReadFile": {"allow", "deny"},
    "beforeTabFileRead": {"allow", "deny"},
    "subagentStart": {"allow", "deny"},
}

TRIP = "ghp_" + "A" * 24
KEY = "-----BEGIN " + "RSA PRIVATE KEY-----"
PROSE = "x = 1\n" + "/" + "/ explain why\n"
FORCE = "git push origin main" + " --force"
RMRF = "rm" + " -rf /"
INSTALL = "npm" + " install lodash"
PROSE_SH = 'printf "%s\\n" "' + "/" + '/ why" >> src/foo.ts'

TRIPWIRES = {
    "beforeSubmitPrompt": [
        {"hook_event_name": "beforeSubmitPrompt", "prompt": f"ship {TRIP}", "attachments": []},
    ],
    "preToolUse": [
        {"hook_event_name": "preToolUse", "tool_name": "Write", "tool_use_id": "t1",
         "cwd": "/tmp", "tool_input": {"path": "src/a.ts", "contents": PROSE}},
        {"hook_event_name": "preToolUse", "tool_name": "Write", "tool_use_id": "t2",
         "cwd": "/tmp", "tool_input": {"path": "src/a.ts", "contents": f'k="{KEY}"\n'}},
        {"hook_event_name": "preToolUse", "tool_name": "Write", "tool_use_id": "t3",
         "cwd": str(PACK / "probe"), "tool_input": {"path": "PROBE_PATH", "contents": "export function Bad_Name() {}\n"}},
        {"hook_event_name": "preToolUse", "tool_name": "Delete", "tool_use_id": "t4",
         "cwd": "/tmp", "tool_input": {"path": "payments", "recursive": True}},
    ],
    "beforeShellExecution": [
        {"hook_event_name": "beforeShellExecution", "cwd": "/tmp", "sandbox": False,
         "command": FORCE},
        {"hook_event_name": "beforeShellExecution", "cwd": "/tmp", "sandbox": False,
         "command": RMRF},
        {"hook_event_name": "beforeShellExecution", "cwd": "/tmp", "sandbox": False,
         "command": INSTALL},
        {"hook_event_name": "beforeShellExecution", "cwd": "/tmp", "sandbox": False,
         "command": PROSE_SH},
    ],
    "beforeMCPExecution": [
        {"hook_event_name": "beforeMCPExecution", "tool_name": "postgres_drop_table",
         "tool_input": {"table": "users"}},
        {"hook_event_name": "beforeMCPExecution", "tool_name": "notes_write",
         "tool_input": {"text": f"token {TRIP}"}},
    ],
    "beforeReadFile": [
        {"hook_event_name": "beforeReadFile", "file_path": ".env", "path": ".env"},
    ],
    "beforeTabFileRead": [
        {"hook_event_name": "beforeTabFileRead", "file_path": "id_rsa", "path": "id_rsa"},
    ],
}


def run(command: str, payload: dict) -> tuple[int, dict | None, str]:
    proc = subprocess.run(
        command, shell=True, cwd=str(PACK),
        input=json.dumps(payload).encode(), capture_output=True,
    )
    raw = proc.stdout.decode().strip()
    if not raw:
        return proc.returncode, None, ""
    try:
        return proc.returncode, json.loads(raw), raw
    except json.JSONDecodeError:
        return proc.returncode, None, raw


def matcher_hit(matcher: str | None, tool_name: str) -> bool:
    if not matcher:
        return True
    return bool(re.search(matcher, tool_name or "", re.I))


def main() -> int:
    cfg = json.loads(HOOKS_JSON.read_text(encoding="utf-8"))
    meters = "".join(
        (HERE / n).read_text(encoding="utf-8")
        for n in (
            "_selftest.py",
            "_proof_evals.py",
            "_verify_hook_contracts.py",
            "_gauntlet.py",
        )
        if (HERE / n).is_file()
    )
    fails: list[str] = []

    tmp = Path(tempfile.mkdtemp())
    vdir = tmp / ".cursor" / "rules"
    vdir.mkdir(parents=True)
    (vdir / "vernacular.mdc").write_text(
        "file_name_pattern: domain.kind.ext\nallowed_kinds: usecase\n"
        "class_pattern: PascalCase\nfunction_pattern: verbObject\n",
        encoding="utf-8",
    )
    (tmp / "src").mkdir()
    probe_path = tmp / "src" / "BadName.ts"

    def fill(payload: dict) -> dict:
        out = json.loads(json.dumps(payload))
        ti = out.get("tool_input")
        if isinstance(ti, dict) and ti.get("path") == "PROBE_PATH":
            ti["path"] = str(probe_path)
            out["cwd"] = str(tmp)
        return out

    for event, entries in cfg.get("hooks", {}).items():
        if event not in KNOWN_EVENTS:
            fails.append(f"[UNKNOWN-EVENT] {event} is not a platform hook event")
            continue
        allowed = OUTPUT_FIELDS.get(event)
        wires = TRIPWIRES.get(event)

        for entry in entries:
            command = entry["command"]
            name = command.split("/")[-1]
            local = command.replace("./hooks/", str(HERE) + "/")
            matcher = entry.get("matcher")

            if name not in meters:
                fails.append(f"[UNCOVERED] {event}:{name} asserted by no meter")

            if allowed is None:
                continue
            if wires is None:
                if matcher:
                    fails.append(f"[UNPROBED-MATCHER] {event}:{name} matcher={matcher!r} has no tripwires")
                continue

            matched = [w for w in wires if matcher_hit(matcher, str(w.get("tool_name") or ""))]
            if matcher and not matched:
                fails.append(
                    f"[UNPROBED-MATCHER] {event}:{name} matcher={matcher!r} "
                    f"matched none of {len(wires)} tripwire(s)"
                )
                continue

            field = BLOCK_FIELD.get(event)
            blocked_any = False

            for wire in matched:
                code, obj, raw = run(local, fill(wire))
                if obj is None and raw:
                    fails.append(f"[NON-JSON] {event}:{name} emitted non-JSON on stdout")
                    continue
                if obj is None:
                    continue

                stray = sorted(set(obj) - allowed)
                if stray:
                    fails.append(
                        f"[SCHEMA-DRIFT] {event}:{name} emits {stray} "
                        f"which {event} does not read"
                    )

                perm = obj.get("permission")
                enforced = ENFORCED_PERMISSIONS.get(event)
                if perm and enforced and perm not in enforced:
                    fails.append(
                        f"[UNENFORCED-VERB] {event}:{name} returns "
                        f"permission={perm!r}, not enforced for {event}"
                    )

                if field is None:
                    continue

                hit = (obj.get("continue") is False) if field == "continue" \
                    else (perm in {"deny", "ask"})
                blocked_any = blocked_any or hit or code == 2

            if field is not None and matched and not blocked_any:
                fails.append(
                    f"[DEAD-GATE] {event}:{name} blocked none of "
                    f"{len(matched)} tripwire(s) via `{field}` or exit 2"
                )

    if fails:
        for line in fails:
            print(line)
        print(f"[FAIL] {len(fails)} hook-contract violation(s)")
        return 1
    print("ALL_HOOK_CONTRACTS_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
