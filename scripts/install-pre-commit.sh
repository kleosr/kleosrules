#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/.git/hooks/pre-commit"
BIN="$ROOT/hooks/bin/kleos-gate"
cat >"$HOOK" <<EOF
#!/usr/bin/env bash
set -euo pipefail
ROOT="\$(git rev-parse --show-toplevel)"
export KLEOS_HOOKS_DIR="\$ROOT/hooks"
export KLEOS_POLICY_DIR="\$ROOT/hooks/policy"
exec "\$ROOT/hooks/bin/kleos-gate" gate-diff
EOF
chmod +x "$HOOK"
[[ -x "$BIN" ]] || { echo "missing $BIN — run cargo build --release first"; exit 1; }
echo "[ok] pre-commit → hooks/bin/kleos-gate gate-diff"
