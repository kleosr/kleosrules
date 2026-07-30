#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STATE="$ROOT/state"
mkdir -p "$STATE"
INPUT="$(cat)"
PROMPT="$(echo "$INPUT" | jq -r '.prompt // .user_prompt // .message // .text // empty' 2>/dev/null || true)"
ROUTE="other"
echo "$PROMPT" | grep -qiE 'implement|fix|refactor|bug|code|patch|cargo|typescript|rust|bash' && ROUTE="code"
echo "$PROMPT" | grep -qiE 'remember|vault|obsidian|handoff|amnesia|session' && ROUTE="memory"
printf '%s\n' "$PROMPT" >"$STATE/current_intent.md"
DUTY="$(jq -r '.duty // empty' "$ROOT/hooks/policy/intent.json" 2>/dev/null || true)"
[[ -z "$DUTY" ]] && DUTY="Thin INTENT: OBJECTIVE + optional local CONSTRAINTS + deterministic Done-when. Never rewrite user prompt."
jq -n --arg c "ROUTE_CLASSIFY: ${ROUTE}
DEBERES: ${DUTY}
Amnesia: vault_read wiki/hot.md then wiki/index.md; end Session wiki/projects/<slug>/Sessions/YYYY-MM-DD-topic + hot." '{additional_context: $c}'
