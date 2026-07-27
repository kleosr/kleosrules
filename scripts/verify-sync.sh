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

HOOK_NEED=(
  bin/kleos-gate
  policy/shell.json
  policy/lean.json
  policy/secrets.json
  policy/ask-scope.json
)

check_project_hooks() {
  local root="$1" label="$2"
  local hj="$root/.cursor/hooks.json"
  local hd="$root/.cursor/hooks"
  local f
  if [[ ! -f "$hj" ]]; then
    echo "[MISSING] $label/.cursor/hooks.json"
    fail=1
    return
  fi
  if ! grep -q 'kleos-gate' "$hj"; then
    echo "[DRIFT] $label hooks.json missing kleos-gate"
    fail=1
  fi
  if ! grep -q 'Write|StrReplace|EditNotebook' "$hj"; then
    echo "[DRIFT] $label hooks.json write matcher"
    fail=1
  fi
  if grep -q 'python3' "$hj"; then
    echo "[DRIFT] $label hooks.json still references python3"
    fail=1
  fi
  if [[ ! -d "$hd" ]]; then
    echo "[MISSING] $label/.cursor/hooks"
    fail=1
    return
  fi
  for f in "${HOOK_NEED[@]}"; do
    if [[ ! -e "$hd/$f" ]]; then
      echo "[MISSING] $label/.cursor/hooks/$f"
      fail=1
    fi
  done
  if [[ -f "$hd/bin/kleos-gate" && ! -x "$hd/bin/kleos-gate" ]]; then
    echo "[DRIFT] $label kleos-gate not executable"
    fail=1
  fi
  if ! cmp -s "$SSOT/hooks/hooks.project.json" "$hj"; then
    echo "[DRIFT] $label/.cursor/hooks.json"
    fail=1
  fi
}

echo "[scan] verifying ${#REPOS[@]} project(s)"
for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  [[ "$(realpath "$repo_path")" == "$(realpath "$SSOT")" ]] && continue
  check_shared_dest "$repo_path/.cursor/rules" "$(basename "$repo_path")" "copy"
  check_project_hooks "$repo_path" "$(basename "$repo_path")"
done

check_project_hooks "$SSOT" "pack"

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
  hooks/bin/kleos-gate \
  hooks/policy/shell.json \
  hooks/policy/lean.json \
  hooks/policy/secrets.json \
  hooks/policy/ask-scope.json \
  hooks/kleos-gate/Cargo.toml \
  hooks/kleos-gate/tests/integration.rs \
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
