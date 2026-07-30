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
LOOP="$(echo "$INPUT" | jq -r '.loop_count // .loopCount // 0' 2>/dev/null || echo 0)"
MSG_N="$(echo "$INPUT" | jq -r '((.messages // .transcript // .conversation // []) | if type=="array" then length else 0 end)' 2>/dev/null || echo 0)"
LAST="$(echo "$INPUT" | jq -r '
  ((.messages // .transcript // .conversation // []) | if type=="array" then . else [] end)
  | map(select((.role // .type // "") | test("assistant"; "i")))
  | last
  | (.content // .text // empty)
  | if type=="array" then map(.text // .content // tostring) | join("\n") else tostring end
' 2>/dev/null || true)"
if [[ -z "$LAST" || "$LAST" == "null" ]]; then
  LAST="$TEXT"
fi
follow() {
  jq -n --arg m "$1" '{followup_message: $m}'
  exit 0
}
quiet() {
  echo '{}'
  exit 0
}
if echo "$TEXT" | grep -Fq 'STOP ACCEPTED'; then
  quiet
fi
accept() {
  local date
  date="$(date +%Y-%m-%d)"
  rm -rf "$STATE"
  mkdir -p "$STATE"
  printf '%s\n' "TAREA: Session complete ${date}" "ARCHIVOS: (agent fills)" "ESTADO: Done-when met" "SIGUIENTE: vault_write Session + hot" "INTENT: (see chat)" >"$HANDOFF"
  if [[ "${LOOP:-0}" -ge 1 ]]; then
    quiet
  fi
  follow "STOP ACCEPTED. Obsidian write-back: vault_write wiki/projects/<slug>/Sessions/${date}-topic.md COMPLETE (Goal Done-when Residual LAYER CHECK) + refresh wiki/hot.md + mirror HANDOFF.md. Close with Done-when: met if not already written."
}
if [[ "${MSG_N:-0}" -eq 0 ]] && ! echo "$TEXT" | grep -q 'INTENT:'; then
  accept
fi
HAS_DONE=0
HAS_INTENT=0
echo "$TEXT" | grep -q 'Done-when:' && HAS_DONE=1
echo "$TEXT" | grep -q 'INTENT:' && HAS_INTENT=1
if [[ "$HAS_DONE" -eq 0 || "$HAS_INTENT" -eq 0 ]]; then
  follow "INTENT required: restate INTENT: and Done-when: in chat. Do not rewrite the user prompt. Then finish Done-when fully (no partial handoff). Close with Done-when: met, then Obsidian Session + hot + HANDOFF."
fi
HAS_MET=0
echo "$TEXT" | grep -iqE 'Done-when[[:space:]]*:[[:space:]]*(met|cumplido|complete|done)\b|✅[[:space:]]*Done-when[[:space:]]+met' && HAS_MET=1
if [[ "$HAS_MET" -eq 0 ]]; then
  if echo "$LAST" | grep -iqE '(déjame saber|quieres que|puedo (hacer|agregar)|me avisas|debería agregar|let me know if|want me to|should I (add|also)|I can (add|also)|if you('\''|’)d like|if you want( me)? to|say if you want)'; then
    follow "STOP REJECTED: partial handoff / permission ask inside INTENT. Do not ask. Execute the missing Done-when work NOW, then write Done-when: met and stop again."
  fi
  follow "PREMATURE STOP: Done-when not confirmed met. Finish every Done-when point (cascade-fix red builds). When complete, write Done-when: met then stop."
fi
accept
