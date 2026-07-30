#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HANDOFF="$ROOT/HANDOFF.md"
TAIL=""
if [[ -f "$HANDOFF" ]]; then
  TAIL="$(tail -n 15 "$HANDOFF")"
fi
CTX="AMNESIA: vault survives. vault_read wiki/hot.md then wiki/index.md. Thin INTENT: one OBJECTIVE + optional local CONSTRAINTS + deterministic Done-when (≤5 anchors). HANDOFF tail:
${TAIL}"
jq -n --arg c "$CTX" '{additional_context: $c}'
