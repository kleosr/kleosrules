#!/usr/bin/env bash
# sync-to-repos.sh — pack → discovered projects + ~/.cursor/skills
set -euo pipefail

SSOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
personal_skills="${HOME}/.cursor/skills"
RULES_DIR="$SSOT/project-rules"
SHARED=(agent types testing debugging native-lean-autoload ponytail lean-code)
export SSOT

# shellcheck source=../lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

mapfile -t RETIRED < <(grep -v '^\s*#' "$SSOT/config/retired.txt" | grep -v '^\s*$' || true)
mapfile -t SKILLS < <(grep -v '^\s*#' "$SSOT/config/skills.txt" | grep -v '^\s*$' || true)
mapfile -t RETIRED_SKILLS < <(grep -v '^\s*#' "$SSOT/config/retired-skills.txt" | grep -v '^\s*$' || true)
mapfile -t REPOS < <(discover | sort -u)

link_ssot_cursor() {
  local dest="$SSOT/.cursor/rules"
  mkdir -p "$dest"
  local name
  for name in "${SHARED[@]}"; do
    local src="$RULES_DIR/${name}.mdc"
    [[ -f "$src" ]] || { echo "[fail] missing $src"; exit 1; }
    ln -sfn "../../project-rules/${name}.mdc" "$dest/${name}.mdc"
  done
  local orphan
  for orphan in "${RETIRED[@]}"; do
    if [[ -e "$dest/$orphan" || -L "$dest/$orphan" ]]; then
      rm -f "$dest/$orphan"
      echo "[rm]  $orphan"
    fi
  done
  echo "[ok]  pack .cursor/rules → project-rules/"
}

sync_into() {
  local dest="$1"
  local label="$2"
  mkdir -p "$dest"
  local name
  for name in "${SHARED[@]}"; do
    local src="$RULES_DIR/${name}.mdc"
    [[ -f "$src" ]] || { echo "[fail] missing $src"; exit 1; }
    [[ -L "$dest/${name}.mdc" ]] && rm -f "$dest/${name}.mdc"
    cp -f "$src" "$dest/${name}.mdc"
  done
  local orphan
  for orphan in "${RETIRED[@]}"; do
    if [[ -e "$dest/$orphan" || -L "$dest/$orphan" ]]; then
      rm -f "$dest/$orphan"
      echo "[rm]  $label/$orphan"
    fi
  done
  echo "[ok]  $label"
}

link_personal_skills() {
  local skill src dst
  mkdir -p "$personal_skills"
  for skill in "${SKILLS[@]}"; do
    src="$SSOT/skills/$skill"
    dst="$personal_skills/$skill"
    [[ -f "$src/SKILL.md" ]] || { echo "[fail] missing $src/SKILL.md"; exit 1; }
    if [[ -e "$dst" && ! -L "$dst" ]]; then
      if [[ "${FORCE_SKILLS:-0}" == "1" ]]; then
        rm -rf "$dst"
      else
        echo "[warn] skip non-symlink: $dst"
        continue
      fi
    fi
    ln -sfn "$(realpath --relative-to="$personal_skills" "$src")" "$dst"
  done
  for skill in "${RETIRED_SKILLS[@]}"; do
    dst="$personal_skills/$skill"
    [[ -L "$dst" ]] && rm -f "$dst"
  done
  echo "[ok]  skills → pack/skills"
}

echo "[scan] discovered ${#REPOS[@]} project(s)"
for local_r in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${local_r:-}" ]] && continue
  echo "[scan]  - $local_r"
done

link_personal_skills
link_ssot_cursor

for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  if [[ "$(realpath "$repo_path")" == "$(realpath "$SSOT")" ]]; then
    continue
  fi
  sync_into "$repo_path/.cursor/rules" "$(basename "$repo_path")"
done

echo "Sync complete. Verify: bash $SSOT/scripts/verify-sync.sh"
