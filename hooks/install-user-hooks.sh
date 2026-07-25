#!/usr/bin/env bash
# install-user-hooks.sh — copy hooks into ~/.cursor
set -euo pipefail
PACK="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$PACK/hooks"
DEST="${HOME}/.cursor"
mkdir -p "$DEST/hooks"

for f in \
  block-dangerous-git.sh \
  deny-danger.sh \
  ask-gated-shell.sh \
  prose_comment_lib.py \
  deny-prose-comments.py \
  deny-shell-prose-writes.py \
  deny-vernacular-drift.py \
  scan-edited-file-for-prose.py \
  block-secrets.py
do
  [[ -f "$SRC/$f" ]] || { echo "[fail] missing $SRC/$f"; exit 1; }
  cp -f "$SRC/$f" "$DEST/hooks/$f"
  chmod +x "$DEST/hooks/$f"
done

cp -f "$SRC/hooks.json" "$DEST/hooks.json"
echo "[ok] hooks → $DEST/hooks.json"
