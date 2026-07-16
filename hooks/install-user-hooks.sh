#!/usr/bin/env bash
# install-user-hooks.sh — install SSOT hooks into ~/.cursor (user-global SAFETY).
set -euo pipefail
SSOT="/home/kleosr/Documentos/rules"
SRC="$SSOT/hooks"
DEST="${HOME}/.cursor"
mkdir -p "$DEST/hooks"
cp -f "$SRC/block-dangerous-git.sh" "$DEST/hooks/block-dangerous-git.sh"
chmod +x "$DEST/hooks/block-dangerous-git.sh"

# User hooks.json paths are relative to ~/.cursor/
cat >"$DEST/hooks.json" <<'EOF'
{
  "version": 1,
  "hooks": {
    "beforeShellExecution": [
      {
        "command": "./hooks/block-dangerous-git.sh",
        "failClosed": true
      }
    ]
  }
}
EOF
echo "[ok] user hooks installed → $DEST/hooks.json"
