#!/usr/bin/env bash
set -euo pipefail
SSOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export SSOT
# shellcheck source=../lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

HOOK_FILES=(
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

sync_hooks_into() {
  local root="$1" label="$2"
  local dest="$root/.cursor/hooks"
  mkdir -p "$dest"
  local f
  for f in "${HOOK_FILES[@]}"; do
    cp -f "$SSOT/hooks/$f" "$dest/$f"
    chmod +x "$dest/$f"
  done
  cp -f "$SSOT/hooks/hooks.project.json" "$root/.cursor/hooks.json"
  echo "[ok]  hooks → $label"
}

sync_hooks_into "$SSOT" "pack"
mapfile -t REPOS < <(discover | sort -u)
for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  [[ "$(realpath "$repo_path")" == "$(realpath "$SSOT")" ]] && continue
  sync_hooks_into "$repo_path" "$(basename "$repo_path")"
done
