#!/usr/bin/env bash
# verify-sync.sh — pack integrity
set -euo pipefail

SSOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
personal_skills="${HOME}/.cursor/skills"
RULES_DIR="$SSOT/project-rules"
SHARED=(agent types testing debugging native-lean-autoload ponytail lean-code)
fail=0
export SSOT

# shellcheck source=../lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

mapfile -t RETIRED < <(grep -v '^\s*#' "$SSOT/config/retired.txt" | grep -v '^\s*$' || true)
mapfile -t SKILLS < <(grep -v '^\s*#' "$SSOT/config/skills.txt" | grep -v '^\s*$' || true)
mapfile -t RETIRED_SKILLS < <(grep -v '^\s*#' "$SSOT/config/retired-skills.txt" | grep -v '^\s*$' || true)
mapfile -t REPOS < <(discover | sort -u)

check_shared_dest() {
  local dest="$1" label="$2" mode="$3"
  local name src dst
  if [[ ! -d "$dest" ]]; then
    echo "[MISSING] $label/.cursor/rules"
    fail=1
    return
  fi
  for name in "${SHARED[@]}"; do
    src="$RULES_DIR/${name}.mdc"
    dst="$dest/${name}.mdc"
    if [[ ! -e "$dst" && ! -L "$dst" ]]; then
      echo "[MISSING] $label/${name}.mdc"
      fail=1
      continue
    fi
    if [[ "$mode" == "symlink" ]]; then
      if [[ ! -L "$dst" ]] || ! cmp -s "$src" "$dst"; then
        echo "[BAD] $label/${name}.mdc symlink"
        fail=1
      fi
    else
      if [[ -L "$dst" ]] || ! cmp -s "$src" "$dst"; then
        echo "[DRIFT] $label/${name}.mdc"
        fail=1
      fi
    fi
  done
  local orphan
  for orphan in "${RETIRED[@]}"; do
    if [[ -e "$dest/$orphan" || -L "$dest/$orphan" ]]; then
      echo "[ORPHAN] $label/$orphan"
      fail=1
    fi
  done
}

check_shared_dest "$SSOT/.cursor/rules" "pack" "symlink"

echo "[scan] verifying ${#REPOS[@]} project(s)"
for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  [[ "$(realpath "$repo_path")" == "$(realpath "$SSOT")" ]] && continue
  check_shared_dest "$repo_path/.cursor/rules" "$(basename "$repo_path")" "copy"
done

for skill in "${SKILLS[@]}"; do
  src="$SSOT/skills/$skill"
  dst="$personal_skills/$skill"
  if [[ ! -f "$src/SKILL.md" ]]; then
    echo "[MISSING] skills/$skill/SKILL.md"
    fail=1
  elif [[ ! -L "$dst" ]] || [[ "$(realpath "$dst")" != "$(realpath "$src")" ]]; then
    echo "[BAD-LINK] ~/.cursor/skills/$skill"
    fail=1
  fi
done

for req in \
  user-rules/USER-RULES.paste.txt \
  user-rules/option-c-core.mdc \
  project-rules/agent.mdc \
  project-rules/native-lean-autoload.mdc \
  project-rules/ponytail.mdc \
  project-rules/lean-code.mdc \
  hooks/hooks.json \
  hooks/hooks.project.json \
  hooks/deny-prose-comments.py \
  hooks/deny-shell-prose-writes.py \
  hooks/deny-vernacular-drift.py \
  hooks/gate-write.py \
  hooks/gate-read.py \
  hooks/gate-mcp.py \
  hooks/gate-delete.py \
  hooks/hookio.py \
  hooks/ask-gated-shell.sh \
  config/skills.txt \
  install.sh
do
  [[ -e "$SSOT/$req" ]] || { echo "[MISSING] $req"; fail=1; }
done

# root must stay clean
for junk in agent.mdc types.mdc USER-RULES.paste.txt option-c-core.mdc skills.txt scan.roots glob; do
  if [[ -e "$SSOT/$junk" ]]; then
    echo "[ROOT-MESS] $junk belongs in a subfolder"
    fail=1
  fi
done

if [[ $fail -eq 0 ]]; then
  echo "[PASS] organized pack OK"
else
  echo "[FAIL] organization or sync broken"
  exit 1
fi
