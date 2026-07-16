#!/usr/bin/env bash
# discover-repos.sh — print discovered project roots (one absolute path per line).
# Sourced or executed. Does not require a static fleet list.
set -euo pipefail

SSOT="${SSOT:-/home/kleosr/Documentos/rules}"
ROOTS_FILE="${SSOT}/scan.roots"
IGNORE_FILE="${SSOT}/scan.ignore"

# Default scan roots if scan.roots missing/empty
default_roots() {
  printf '%s\n' "/home/kleosr/Documentos"
}

load_lines() {
  local f="$1"
  [[ -f "$f" ]] || return 0
  grep -v '^\s*#' "$f" | grep -v '^\s*$' || true
}

is_ignored() {
  local path="$1" base
  base="$(basename "$path")"
  # Always ignore the SSOT harness itself (handled separately)
  [[ "$path" == "$SSOT" ]] && return 0
  [[ "$base" == "rules" && "$(dirname "$path")" == "/home/kleosr/Documentos" ]] && return 0

  local pat
  while IFS= read -r pat; do
    [[ -z "$pat" ]] && continue
    # basename match or substring path match
    if [[ "$base" == "$pat" || "$path" == *"$pat"* ]]; then
      return 0
    fi
  done < <(load_lines "$IGNORE_FILE")
  return 1
}

is_project() {
  local d="$1"
  [[ -d "$d" ]] || return 1
  # Project signals (any one is enough)
  [[ -d "$d/.git" ]] && return 0
  [[ -f "$d/package.json" ]] && return 0
  [[ -f "$d/pnpm-workspace.yaml" ]] && return 0
  [[ -f "$d/Cargo.toml" ]] && return 0
  [[ -f "$d/go.mod" ]] && return 0
  [[ -f "$d/pyproject.toml" ]] && return 0
  [[ -f "$d/AGENTS.md" ]] && return 0
  [[ -d "$d/.cursor/rules" ]] && return 0
  return 1
}

discover() {
  local roots=()
  local line
  if [[ -f "$ROOTS_FILE" ]] && [[ -n "$(load_lines "$ROOTS_FILE" | head -1 || true)" ]]; then
    while IFS= read -r line; do
      roots+=("$line")
    done < <(load_lines "$ROOTS_FILE")
  else
    while IFS= read -r line; do
      roots+=("$line")
    done < <(default_roots)
  fi

  local root child
  for root in "${roots[@]}"; do
    [[ -d "$root" ]] || continue
    # If the root itself is a project (rare), include it
    if is_project "$root" && ! is_ignored "$root"; then
      printf '%s\n' "$root"
    fi
    # Immediate children only (Documentos/<repo>) — predictable, no deep crawl
    shopt -s nullglob
    for child in "$root"/*/; do
      child="${child%/}"
      is_ignored "$child" && continue
      is_project "$child" || continue
      printf '%s\n' "$child"
    done
    shopt -u nullglob
  done
}

# When executed (not sourced), print discoveries
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  discover | sort -u
fi
