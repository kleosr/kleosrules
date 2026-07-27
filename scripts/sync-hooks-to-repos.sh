#!/usr/bin/env bash
set -euo pipefail
SSOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export SSOT
# shellcheck source=../lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

if [[ ! -x "$SSOT/hooks/bin/kleos-gate" ]]; then
  (cd "$SSOT/hooks/kleos-gate" && cargo build --release)
  mkdir -p "$SSOT/hooks/bin"
  cp -f "$SSOT/hooks/kleos-gate/target/release/kleos-gate" "$SSOT/hooks/bin/kleos-gate"
  chmod +x "$SSOT/hooks/bin/kleos-gate"
fi

sync_hooks_into() {
  local root="$1" label="$2"
  local dest="$root/.cursor/hooks"
  mkdir -p "$dest/bin" "$dest/policy"
  cp -f "$SSOT/hooks/bin/kleos-gate" "$dest/bin/kleos-gate"
  chmod +x "$dest/bin/kleos-gate"
  cp -f "$SSOT/hooks/policy/shell.json" "$SSOT/hooks/policy/lean.json" \
    "$SSOT/hooks/policy/ask-scope.json" "$SSOT/hooks/policy/secrets.json" \
    "$dest/policy/"
  cp -f "$SSOT/hooks/hooks.project.json" "$root/.cursor/hooks.json"
  echo "[ok]  hooks → $label (kleos-gate)"
}

sync_hooks_into "$SSOT" "pack"
mapfile -t REPOS < <(discover | sort -u)
for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  [[ "$(realpath "$repo_path")" == "$(realpath "$SSOT")" ]] && continue
  sync_hooks_into "$repo_path" "$(basename "$repo_path")"
done
