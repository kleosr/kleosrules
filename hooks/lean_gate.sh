#!/usr/bin/env bash
set -euo pipefail
INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty')
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.filePath // .tool_input.path // empty')
if [[ "$TOOL_NAME" != "Write" && "$TOOL_NAME" != "StrReplace" ]]; then
  echo '{"action":"allow"}'
  exit 0
fi
if [[ -z "$FILE_PATH" ]]; then
  echo '{"action":"allow"}'
  exit 0
fi
if [[ -f "$FILE_PATH" ]]; then
  LINES=$(wc -l < "$FILE_PATH")
  if [[ "$LINES" -gt 700 ]]; then
    echo "{\"action\":\"deny\",\"user_message\":\"MECHANICAL DENY: file exceeds 700 LOC ($LINES). Split.\"}"
    exit 2
  fi
fi
echo '{"action":"allow"}'
exit 0
