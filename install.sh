#!/usr/bin/env bash
# install.sh — install kleosrules into ~/.cursor
set -euo pipefail
PACK="$(cd "$(dirname "$0")" && pwd)"
DEST="${HOME}/.cursor"
mkdir -p "$DEST/rules" "$DEST/skills" "$DEST/hooks"

echo "==> Hooks"
bash "$PACK/hooks/install-user-hooks.sh"

echo "==> Rules mirror (always-on global companions)"
cp -f "$PACK/user-rules/option-c-core.mdc" "$DEST/rules/option-c-core.mdc"
for f in native-lean-autoload ponytail lean-code agent; do
  cp -f "$PACK/project-rules/${f}.mdc" "$DEST/rules/${f}.mdc"
  echo "[ok]  ~/.cursor/rules/${f}.mdc (alwaysApply)"
done

echo "==> Skills (symlinks → pack/skills)"
mapfile -t SKILLS < <(grep -v '^\s*#' "$PACK/config/skills.txt" | grep -v '^\s*$' || true)
for skill in "${SKILLS[@]}"; do
  src="$PACK/skills/$skill"
  dst="$DEST/skills/$skill"
  [[ -f "$src/SKILL.md" ]] || { echo "[fail] missing $src/SKILL.md"; exit 1; }
  if [[ -e "$dst" && ! -L "$dst" ]]; then
    if [[ "${FORCE_SKILLS:-0}" == "1" ]]; then
      rm -rf "$dst"
      echo "[force] replaced: $skill"
    else
      echo "[warn] skip non-symlink: $dst  (FORCE_SKILLS=1 to replace)"
      continue
    fi
  fi
  ln -sfn "$(realpath --relative-to="$DEST/skills" "$src")" "$dst"
  echo "[ok]  $skill"
done

echo
echo "==> USER RULES (manual paste required)"
echo "    Cursor → Settings → Rules → User Rules"
echo "    Paste: $PACK/user-rules/USER-RULES.paste.txt"
echo
echo "==> Vernacular (per app repo — soft until alwaysApply contract exists)"
echo "    mkdir -p .cursor/rules"
echo "    cp \"$PACK/skills/vernacular/TEMPLATE.md\" .cursor/rules/vernacular.mdc"
echo
echo "[done] $PACK"
echo "       New chat after User Rules paste."
