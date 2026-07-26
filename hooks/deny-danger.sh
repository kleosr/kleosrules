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
deny() {
  python3 -c 'import json,sys; print(json.dumps({"permission":"deny","user_message":sys.argv[1],"agent_message":sys.argv[1]}))' "$1"
  exit 2
}
[[ -z "$cmd" ]] && { echo '{"permission":"allow"}'; exit 0; }
low=$(printf '%s' "$cmd" | tr '[:upper:]' '[:lower:]')
case "$low" in
  *"rm -rf /"*|*"rm -fr /"*|*"rm -rf /*"*|*"rm -fr /*"*|*"rm -rf ~"*)
    deny "Blocked destructive rm (lab gate)" ;;
esac
case "$low" in
  *"git push"*"--force"*|*"git push"*"--force-with-lease"*|*"git push -f "*)
    deny "Blocked force-push (lab gate)" ;;
esac
case "$low" in
  *"git reset --hard"*|*"git clean -fdx"*)
    deny "Blocked destructive git reset/clean (lab gate)" ;;
esac
case "$low" in
  *"curl"*"| sh"*|*"curl"*"| bash"*|*"wget"*"| sh"*|*"wget"*"| bash"*)
    deny "Blocked pipe-to-interpreter (lab gate)" ;;
esac
case "$low" in
  *"mkfs."*|*"dd if="*)
    deny "Blocked disk-destructive command (lab gate)" ;;
esac
case "$low" in
  *"npm publish"*|*"pnpm publish"*|*"terraform destroy"*)
    deny "Blocked publish/destroy (lab gate)" ;;
esac
echo '{"permission":"allow"}'
exit 0
