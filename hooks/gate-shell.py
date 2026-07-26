#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import ask, command_from, deny, emit, load_stdin, log_decision  # noqa: E402
from prose_comment_lib import ask_opaque_payload, deny_payload, shell_write_class  # noqa: E402

DENY_RULES: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"rm\s+-[a-z]*rf\s+/", re.I), "Blocked destructive rm (lab gate)"),
    (re.compile(r"rm\s+-[a-z]*fr\s+/", re.I), "Blocked destructive rm (lab gate)"),
    (re.compile(r"rm\s+-[a-z]*rf\s+/\*", re.I), "Blocked destructive rm (lab gate)"),
    (re.compile(r"rm\s+-[a-z]*fr\s+/\*", re.I), "Blocked destructive rm (lab gate)"),
    (re.compile(r"rm\s+-[a-z]*rf\s+~", re.I), "Blocked destructive rm (lab gate)"),
    (
        re.compile(r"git\s+push[^;&|]*(?:--force|--force-with-lease|\s-f(?:\s|$))", re.I),
        "Blocked force-push (lab gate)",
    ),
    (re.compile(r"git\s+reset\s+--hard", re.I), "Blocked destructive git reset/clean (lab gate)"),
    (re.compile(r"git\s+clean\s+-fdx", re.I), "Blocked destructive git reset/clean (lab gate)"),
    (re.compile(r"curl[^\n]*\|\s*(?:sh|bash)", re.I), "Blocked pipe-to-interpreter (lab gate)"),
    (re.compile(r"wget[^\n]*\|\s*(?:sh|bash)", re.I), "Blocked pipe-to-interpreter (lab gate)"),
    (re.compile(r"mkfs\.", re.I), "Blocked disk-destructive command (lab gate)"),
    (re.compile(r"dd\s+if=", re.I), "Blocked disk-destructive command (lab gate)"),
    (re.compile(r"npm\s+publish", re.I), "Blocked publish/destroy (lab gate)"),
    (re.compile(r"pnpm\s+publish", re.I), "Blocked publish/destroy (lab gate)"),
    (re.compile(r"terraform\s+destroy", re.I), "Blocked publish/destroy (lab gate)"),
    (
        re.compile(r"git\s+(?:commit|push|rebase)[^;&|]*(?:--no-verify|--no-gpg-sign)", re.I),
        "Blocked by harness: no --no-verify / --no-gpg-sign unless user explicitly overrides outside hooks.",
    ),
    (
        re.compile(r"git\s+push[^;&|]*(?:--force| -f\s|--force-with-lease)", re.I),
        "Blocked by harness: no force-push (agent.mdc SAFETY).",
    ),
]

ASK_RULES: list[tuple[re.Pattern[str], str]] = [
    (
        re.compile(r"(?:^|[^a-z])(?:npx\s|npm\s+exec|pnpm\s+dlx|yarn\s+dlx|bunx\s|uvx\s|deno\s+run)", re.I),
        "Untrusted remote runner (npx-class). Confirm package/command list.",
    ),
    (
        re.compile(
            r"(?:^|[^a-z])(?:npm\s+i(?:\s|$)|npm\s+install|npm\s+ci|pnpm\s+install|pnpm\s+add|"
            r"yarn\s+install|yarn\s+add|bun\s+install|pip3?\s+install|cargo\s+add)",
            re.I,
        ),
        "Package install/ci materializes third-party code. Confirm package/command list.",
    ),
    (
        re.compile(r"git\s+push(?:\s|$)", re.I),
        "Remote publish (git push). Confirm remote/ref. MUST-NEVER: no remote publish without confirmation.",
    ),
    (
        re.compile(r"(?:^|[^a-z])(?:gh\s+release\s+create|docker\s+push|podman\s+push)(?:\s|$)", re.I),
        "Remote publish (release/image push). Confirm target. MUST-NEVER: no remote publish without confirmation.",
    ),
    (
        re.compile(r"rm\s+(-[a-zA-Z]*r[a-zA-Z]*f|-[a-zA-Z]*f[a-zA-Z]*r|-[rf]{2})(?:\s|$)", re.I),
        "Recursive rm is destructive (tree wipe class). Confirm exact path list. Surgical single-file delete is ACT; this is not.",
    ),
    (
        re.compile(r"find\s+.*-delete(?:\s|$)|rsync\s+.*--delete", re.I),
        "Mass delete / sync-delete is destructive. Confirm exact path list.",
    ),
    (
        re.compile(r"git\s+reset\s+--hard", re.I),
        "git reset --hard is destructive. Confirm before continuing.",
    ),
    (
        re.compile(r"git\s+clean\s+-[a-zA-Z]*f", re.I),
        "git clean -f is destructive. Confirm before continuing.",
    ),
]


def verdict(cmd: str) -> dict:
    if not cmd:
        return {"permission": "allow"}
    for pat, msg in DENY_RULES:
        if pat.search(cmd):
            return {"permission": "deny", "user_message": msg, "agent_message": msg}
    kind = shell_write_class(cmd)
    if kind == "prose":
        return deny_payload()
    for pat, msg in ASK_RULES:
        if pat.search(cmd):
            return {"permission": "ask", "user_message": msg, "agent_message": msg}
    if kind == "opaque":
        return ask_opaque_payload()
    return {"permission": "allow"}


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        emit(deny_payload(), 2)
    cmd = command_from(data)
    out = verdict(cmd)
    perm = out.get("permission")
    if perm == "deny":
        log_decision("gate-shell", "beforeShellExecution", "deny", cmd[:120])
        emit(out, 2)
    if perm == "ask":
        log_decision("gate-shell", "beforeShellExecution", "ask", cmd[:120])
        emit(out, 0)
    emit(out)


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit(deny_payload(), 2)
