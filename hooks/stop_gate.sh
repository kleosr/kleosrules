#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STATE="$ROOT/state"
HANDOFF="$ROOT/HANDOFF.md"
INPUT="$(cat)"
STATUS="$(echo "$INPUT" | jq -r 'if .status == null then "" else .status end' 2>/dev/null || true)"
if [[ -n "$STATUS" && "$STATUS" != "completed" ]]; then
  echo '{}'
  exit 0
fi
TEXT="$(echo "$INPUT" | jq -r '[.. | strings] | join("\n")' 2>/dev/null || true)"
HAS_DONE=0
HAS_INTENT=0
echo "$TEXT" | grep -q 'Done-when:' && HAS_DONE=1
echo "$TEXT" | grep -q 'INTENT:' && HAS_INTENT=1
if [[ "$HAS_DONE" -eq 0 || "$HAS_INTENT" -eq 0 ]]; then
  MSG="INTENT required: restate INTENT: and Done-when: in chat. Do not rewrite the user prompt. Then Obsidian Session wiki/projects/<slug>/Sessions/YYYY-MM-DD-topic + refresh wiki/hot.md + mirror HANDOFF.md."
  jq -n --arg m "$MSG" '{followup_message: $m}'
  exit 0
fi
rm -rf "$STATE"
mkdir -p "$STATE"
DATE="$(date +%Y-%m-%d)"
printf '%s\n' "TAREA: Session complete ${DATE}" "ARCHIVOS: (agent fills)" "ESTADO: stopped with INTENT+Done-when" "SIGUIENTE: vault_write Session + hot" "INTENT: (see chat)" >"$HANDOFF"
MSG="Obsidian write-back: vault_write wiki/projects/<slug>/Sessions/${DATE}-topic.md COMPLETE (Goal Done-when Residual LAYER CHECK) + refresh wiki/hot.md. Amnesia preventiva."
jq -n --arg m "$MSG" '{followup_message: $m}'
