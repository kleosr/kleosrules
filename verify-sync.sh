#!/usr/bin/env bash
# verify-sync.sh — SSOT integrity + discovered projects match + skills links
set -euo pipefail

SSOT="/home/kleosr/Documentos/rules"
personal_skills="${HOME}/.cursor/skills"
SHARED=(agent types testing debugging)
fail=0
export SSOT

# shellcheck source=lib/discover-repos.sh
source "$SSOT/lib/discover-repos.sh"

mapfile -t RETIRED < <(grep -v '^\s*#' "$SSOT/retired.txt" | grep -v '^\s*$' || true)
mapfile -t SKILLS < <(grep -v '^\s*#' "$SSOT/skills.txt" | grep -v '^\s*$' || true)
mapfile -t RETIRED_SKILLS < <(grep -v '^\s*#' "$SSOT/retired-skills.txt" | grep -v '^\s*$' || true)
mapfile -t REPOS < <(discover | sort -u)

check_shared_dest() {
  local dest="$1"
  local label="$2"
  local mode="$3" # symlink | copy
  local name src dst

  if [[ ! -d "$dest" ]]; then
    echo "[MISSING] $label/.cursor/rules (directory)"
    fail=1
    return
  fi

  for name in "${SHARED[@]}"; do
    src="$SSOT/${name}.mdc"
    dst="$dest/${name}.mdc"
    if [[ ! -e "$dst" && ! -L "$dst" ]]; then
      echo "[MISSING] $label/${name}.mdc"
      fail=1
      continue
    fi
    if [[ "$mode" == "symlink" ]]; then
      if [[ ! -L "$dst" ]]; then
        echo "[NOT-LINK] $label/${name}.mdc (expected symlink to ../../${name}.mdc)"
        fail=1
      elif [[ "$(readlink "$dst")" != "../../${name}.mdc" ]]; then
        echo "[BAD-LINK] $label/${name}.mdc -> $(readlink "$dst")"
        fail=1
      elif ! cmp -s "$src" "$dst"; then
        echo "[BROKEN] $label/${name}.mdc symlink target unreadable"
        fail=1
      fi
    else
      if [[ -L "$dst" ]]; then
        echo "[UNEXPECTED-LINK] $label/${name}.mdc (expected real copy)"
        fail=1
      elif ! cmp -s "$src" "$dst"; then
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

# SSOT harness itself
check_shared_dest "$SSOT/.cursor/rules" "rules" "symlink"

# Discovered projects
echo "[scan] verifying ${#REPOS[@]} discovered project(s)"
for repo_path in "${REPOS[@]+"${REPOS[@]}"}"; do
  [[ -z "${repo_path:-}" ]] && continue
  label="$(basename "$repo_path")"
  check_shared_dest "$repo_path/.cursor/rules" "$label" "copy"
done

# Skills
for skill in "${SKILLS[@]}"; do
  src="$SSOT/skills/$skill"
  dst="$personal_skills/$skill"
  if [[ ! -f "$src/SKILL.md" ]]; then
    echo "[MISSING] canonical skill: $skill/SKILL.md"
    fail=1
  elif [[ ! -L "$dst" ]]; then
    echo "[NOT-LINK] personal skill: $skill"
    fail=1
  elif [[ "$(readlink "$dst")" != "../../Documentos/rules/skills/$skill" ]]; then
    echo "[BAD-LINK] personal skill: $skill -> $(readlink "$dst")"
    fail=1
  fi
done

for skill in "${RETIRED_SKILLS[@]}"; do
  if [[ -e "$SSOT/skills/$skill" || -L "$SSOT/skills/$skill" ]]; then
    echo "[ORPHAN] retired canonical skill: $skill"
    fail=1
  fi
  if [[ -e "$personal_skills/$skill" || -L "$personal_skills/$skill" ]]; then
    echo "[ORPHAN] retired personal skill: $skill"
    fail=1
  fi
done

# User Rules paste check (best-effort; Cursor may store cloud-only)
USER_RULES_BODY="$SSOT/USER-RULES.paste.txt"
if [[ -f "$USER_RULES_BODY" ]]; then
  if ! python3 "$SSOT/lib/check-user-rules.py" 2>/dev/null; then
    echo "[WARN] User Rules not detected in local Cursor storage — paste USER-RULES.paste.txt in Cursor Settings → Rules → User Rules"
    # non-fatal: project .mdc still protect owned repos
  else
    echo "[ok]  User Rules detected in local Cursor storage"
  fi
fi

if [[ $fail -eq 0 ]]; then
  echo "[PASS] rules + personal Skills canonical; discovered projects match; manifests are SSOT"
else
  echo "[FAIL] missing rules, drift, bad links, or retired artifacts"
  exit 1
fi
