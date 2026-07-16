#!/usr/bin/env bash
# sync-to-repos.sh — SSOT Documentos/rules → discovered projects + personal Skills
# Discovery replaces the old static repos.txt fleet.
set -euo pipefail

SSOT="/home/kleosr/Documentos/rules"
personal_skills="${HOME}/.cursor/skills"
SHARED=(agent types testing debugging)
export SSOT

# shellcheck source=lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

mapfile -t RETIRED < <(grep -v '^\s*#' "$SSOT/retired.txt" | grep -v '^\s*$' || true)
mapfile -t SKILLS < <(grep -v '^\s*#' "$SSOT/skills.txt" | grep -v '^\s*$' || true)
mapfile -t RETIRED_SKILLS < <(grep -v '^\s*#' "$SSOT/retired-skills.txt" | grep -v '^\s*$' || true)
mapfile -t REPOS < <(discover | sort -u)

link_ssot_cursor() {
  local dest="$SSOT/.cursor/rules"
  mkdir -p "$dest"
  local name
  for name in "${SHARED[@]}"; do
    local src="$SSOT/${name}.mdc"
    [[ -f "$src" ]] || { echo "[fail] missing $src"; exit 1; }
    ln -sfn "../../${name}.mdc" "$dest/${name}.mdc"
  done
  local orphan
  for orphan in "${RETIRED[@]}"; do
    if [[ -e "$dest/$orphan" || -L "$dest/$orphan" ]]; then
      rm -f "$dest/$orphan"
      echo "[rm]  rules/$orphan"
    fi
  done
  echo "[ok]  rules (.cursor/rules → symlinks)"
}

sync_into() {
  local dest="$1"
  local label="$2"
  mkdir -p "$dest"
  local name
  for name in "${SHARED[@]}"; do
    local src="$SSOT/${name}.mdc"
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
      echo "[fail] personal skill is not a symlink: $dst"
      exit 1
    fi
    ln -sfn "../../Documentos/rules/skills/$skill" "$dst"
  done
  for skill in "${RETIRED_SKILLS[@]}"; do
    dst="$personal_skills/$skill"
    if [[ -L "$dst" ]]; then
      rm -f "$dst"
    elif [[ -e "$dst" ]]; then
      echo "[fail] retired personal skill is not a symlink: $dst"
      exit 1
    fi
  done
  echo "[ok]  personal skills → SSOT symlinks"
}

echo "[scan] discovered ${#REPOS[@]} project(s)"
if [[ ${#REPOS[@]} -eq 0 ]]; then
  echo "[warn] no projects discovered under scan.roots (skills + SSOT links still apply)"
else
  local_r=
  for local_r in "${REPOS[@]}"; do
    echo "[scan]  - $local_r"
  done
fi

link_personal_skills
link_ssot_cursor

for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  label="$(basename "$repo_path")"
  sync_into "$repo_path/.cursor/rules" "$label"
done

echo "Sync complete. Verify: bash $SSOT/verify-sync.sh"
