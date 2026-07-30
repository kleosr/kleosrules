#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HANDOFF="$ROOT/HANDOFF.md"
TAIL=""
if [[ -f "$HANDOFF" ]]; then
  TAIL="$(tail -n 15 "$HANDOFF")"
fi
CTX="AMNESIA: Cursor window dies; Obsidian vault survives. vault_read wiki/hot.md then wiki/index.md. HANDOFF tail:
${TAIL}"
jq -n --arg c "$CTX" '{additional_context: $c}'
