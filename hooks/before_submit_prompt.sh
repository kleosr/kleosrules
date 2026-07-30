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
DUTY="ROUTE_CLASSIFY: ${ROUTE}
DEBERES: Declara INTENT: y Done-when: en el chat antes de tocar codigo. Injection!=Declaration: hooks only add additional_context; never rewrite the user prompt. Amnesia: read wiki/hot.md + wiki/index.md; end with Session wiki/projects/<slug>/Sessions/YYYY-MM-DD-topic + hot."
jq -n --arg c "$DUTY" '{additional_context: $c}'
