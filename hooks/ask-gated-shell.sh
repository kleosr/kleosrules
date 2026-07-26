#!/usr/bin/env bash
set -euo pipefail
input="$(cat)"
if ! python3 -c 'print(1)' >/dev/null 2>&1; then
  printf '%s\n' '{"permission":"deny","user_message":"Hook parser missing (need python3).","agent_message":"Install python3; shell gates cannot run without a JSON parser."}'
  exit 2
fi
cmd="$(printf '%s' "$input" | python3 -c 'import json,sys
try:
 d=json.load(sys.stdin)
except Exception:
 d={}
print(d.get("command") or d.get("cmd") or "")' 2>/dev/null || true)"
ask() {
  python3 -c 'import json,sys; print(json.dumps({"permission":"ask","user_message":sys.argv[1],"agent_message":sys.argv[1]}))' "$1"
  exit 0
}
[[ -z "$cmd" ]] && { echo '{"permission":"allow"}'; exit 0; }
low=$(printf '%s' "$cmd" | tr '[:upper:]' '[:lower:]')

case "$low" in
  *"npx "*|*"npm exec"*|*"pnpm dlx"*|*"yarn dlx"*|*"bunx "*|*"uvx "*|*"deno run"*)
    ask "Untrusted remote runner (npx-class). Confirm package/command list." ;;
esac

if printf '%s' "$low" | grep -qE '(^|[^a-z])(npm[[:space:]]+i([[:space:]]|$)|npm[[:space:]]+install|npm[[:space:]]+ci|pnpm[[:space:]]+install|pnpm[[:space:]]+add|yarn[[:space:]]+install|yarn[[:space:]]+add|bun[[:space:]]+install|pip3?[[:space:]]+install|cargo[[:space:]]+add)'; then
  ask "Package install/ci materializes third-party code. Confirm package/command list."
fi

if printf '%s' "$low" | grep -qE 'git[[:space:]]+push([[:space:]]|$)'; then
  ask "Remote publish (git push). Confirm remote/ref. MUST-NEVER: no remote publish without confirmation."
fi

if printf '%s' "$low" | grep -qE '(^|[^a-z])(gh[[:space:]]+release[[:space:]]+create|docker[[:space:]]+push|podman[[:space:]]+push)([[:space:]]|$)'; then
  ask "Remote publish (release/image push). Confirm target. MUST-NEVER: no remote publish without confirmation."
fi

if printf '%s' "$low" | grep -qE 'rm[[:space:]]+(-[a-zA-Z]*r[a-zA-Z]*f|-[a-zA-Z]*f[a-zA-Z]*r|-[rf]{2})([[:space:]]|$)'; then
  ask "Recursive rm is destructive (tree wipe class). Confirm exact path list. Surgical single-file delete is ACT; this is not."
fi

if printf '%s' "$low" | grep -qE 'find[[:space:]].*-delete([[:space:]]|$)|rsync[[:space:]].*--delete'; then
  ask "Mass delete / sync-delete is destructive. Confirm exact path list."
fi

echo '{"permission":"allow"}'
exit 0
