#!/usr/bin/env bash
# scan-and-sync.sh — discover → sync → verify
set -euo pipefail
SSOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "=== harness scan $(date -Iseconds) ==="
bash "$SSOT/lib/discover-repos.sh" | sed 's/^/[found] /' || true
bash "$SSOT/scripts/sync-to-repos.sh"
bash "$SSOT/scripts/verify-sync.sh"
echo "=== done ==="
