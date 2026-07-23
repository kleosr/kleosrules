#!/usr/bin/env bash
set -euo pipefail
input=$(cat)
cmd=$(printf '%s' "$input" | jq -r '.command // empty')
deny() { jq -n --arg m "$1" '{permission:"deny",user_message:$m,agent_message:$m}'; exit 0; }
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
