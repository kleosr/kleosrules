#!/usr/bin/env bash
set -euo pipefail
PACK="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$PACK/hooks"
DEST="${HOME}/.cursor"
mkdir -p "$DEST/hooks"

FILES=(
  block-dangerous-git.sh
  deny-danger.sh
  ask-gated-shell.sh
  hookio.py
  prose_comment_lib.py
  deny-prose-comments.py
  deny-shell-prose-writes.py
  deny-vernacular-drift.py
  scan-edited-file-for-prose.py
  block-secrets.py
  gate-write.py
  gate-read.py
  gate-mcp.py
  gate-delete.py
  session-ledger.py
  stop-verify.py
)

for f in "${FILES[@]}"; do
  [[ -f "$SRC/$f" ]] || { echo "[fail] missing $SRC/$f"; exit 1; }
  cp -f "$SRC/$f" "$DEST/hooks/$f"
  chmod +x "$DEST/hooks/$f"
done

cp -f "$SRC/hooks.json" "$DEST/hooks.json"
echo "[ok] hooks → $DEST/hooks.json"
