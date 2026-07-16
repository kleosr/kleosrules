#!/usr/bin/env bash
# block-dangerous-git.sh — beforeShellExecution: deny force-push and hook skips.
# stdin: JSON with .command ; stdout: permission JSON
set -euo pipefail

input="$(cat)"
command=""
if command -v python3 >/dev/null 2>&1; then
  command="$(printf '%s' "$input" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("command") or "")' 2>/dev/null || true)"
fi
if [[ -z "$command" ]]; then
  echo '{"permission":"allow"}'
  exit 0
fi

deny() {
  local msg="$1"
  python3 -c 'import json,sys; print(json.dumps({"permission":"deny","user_message":sys.argv[1],"agent_message":sys.argv[1]}))' "$msg"
  exit 0
}

# Force push variants
if echo "$command" | grep -qiE 'git[[:space:]]+push[^;&|]*(--force| -f[[:space:]]|--force-with-lease)'; then
  deny "Blocked by harness: no force-push (agent.mdc SAFETY)."
fi
if echo "$command" | grep -qiE 'git[[:space:]]+push[^;&|]*\s-f(\s|$)' ; then
  deny "Blocked by harness: no force-push (agent.mdc SAFETY)."
fi

# Skip hooks / skip GPG on commit or push
if echo "$command" | grep -qiE 'git[[:space:]]+(commit|push|rebase)[^;&|]*(--no-verify|--no-gpg-sign)'; then
  deny "Blocked by harness: no --no-verify / --no-gpg-sign unless user explicitly overrides outside hooks."
fi

# Hard reset / clean: ask (user may have requested explicitly in chat)
if echo "$command" | grep -qiE 'git[[:space:]]+reset[[:space:]]+--hard'; then
  python3 -c 'import json; print(json.dumps({"permission":"ask","user_message":"git reset --hard is destructive. Confirm before continuing.","agent_message":"Harness asks confirmation for git reset --hard."}))'
  exit 0
fi
if echo "$command" | grep -qiE 'git[[:space:]]+clean[[:space:]]+-[a-zA-Z]*f'; then
  python3 -c 'import json; print(json.dumps({"permission":"ask","user_message":"git clean -f is destructive. Confirm before continuing.","agent_message":"Harness asks confirmation for git clean -f."}))'
  exit 0
fi

echo '{"permission":"allow"}'
exit 0
