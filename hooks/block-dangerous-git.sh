#!/usr/bin/env bash
set -euo pipefail
input="$(cat)"
if ! python3 -c 'print(1)' >/dev/null 2>&1; then
  printf '%s\n' '{"permission":"deny","user_message":"Hook parser missing (need python3).","agent_message":"Install python3; shell gates cannot run without a JSON parser."}'
  exit 2
fi
command="$(printf '%s' "$input" | python3 -c 'import json,sys
try:
 d=json.load(sys.stdin)
except Exception:
 d={}
print(d.get("command") or d.get("cmd") or "")' 2>/dev/null || true)"

deny() {
  local msg="$1"
  python3 -c 'import json,sys; print(json.dumps({"permission":"deny","user_message":sys.argv[1],"agent_message":sys.argv[1]}))' "$msg"
  exit 2
}

ask() {
  local msg="$1"
  python3 -c 'import json,sys; print(json.dumps({"permission":"ask","user_message":sys.argv[1],"agent_message":sys.argv[1]}))' "$msg"
  exit 0
}

[[ -z "$command" ]] && { echo '{"permission":"allow"}'; exit 0; }

if echo "$command" | grep -qiE 'git[[:space:]]+push[^;&|]*(--force| -f[[:space:]]|--force-with-lease)'; then
  deny "Blocked by harness: no force-push (agent.mdc SAFETY)."
fi
if echo "$command" | grep -qiE 'git[[:space:]]+push[^;&|]*\s-f(\s|$)'; then
  deny "Blocked by harness: no force-push (agent.mdc SAFETY)."
fi

if echo "$command" | grep -qiE 'git[[:space:]]+(commit|push|rebase)[^;&|]*(--no-verify|--no-gpg-sign)'; then
  deny "Blocked by harness: no --no-verify / --no-gpg-sign unless user explicitly overrides outside hooks."
fi

if echo "$command" | grep -qiE 'git[[:space:]]+reset[[:space:]]+--hard'; then
  ask "git reset --hard is destructive. Confirm before continuing."
fi
if echo "$command" | grep -qiE 'git[[:space:]]+clean[[:space:]]+-[a-zA-Z]*f'; then
  ask "git clean -f is destructive. Confirm before continuing."
fi

echo '{"permission":"allow"}'
exit 0
