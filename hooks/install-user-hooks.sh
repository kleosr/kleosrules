#!/usr/bin/env bash
set -euo pipefail
PACK="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$PACK/hooks"
DEST="${HOME}/.cursor"
mkdir -p "$DEST/hooks/bin" "$DEST/hooks/policy"

if [[ ! -x "$SRC/bin/kleos-gate" ]]; then
  (cd "$SRC/kleos-gate" && cargo build --release)
  cp -f "$SRC/kleos-gate/target/release/kleos-gate" "$SRC/bin/kleos-gate"
  chmod +x "$SRC/bin/kleos-gate"
fi

cp -f "$SRC/bin/kleos-gate" "$DEST/hooks/bin/kleos-gate"
chmod +x "$DEST/hooks/bin/kleos-gate"
cp -f "$SRC/policy/shell.json" "$SRC/policy/lean.json" "$SRC/policy/ask-scope.json" "$SRC/policy/secrets.json" "$DEST/hooks/policy/"
cp -f "$SRC/hooks.json" "$DEST/hooks.json"
echo "[ok] hooks → $DEST/hooks.json (kleos-gate)"
