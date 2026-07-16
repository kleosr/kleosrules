#!/usr/bin/env bash
# scan-and-sync.sh — discover projects, sync rules+skills, verify.
# Safe to run periodically (cron / manual). Idempotent.
set -euo pipefail

SSOT="/home/kleosr/Documentos/rules"

echo "=== harness scan $(date -Iseconds) ==="
bash "$SSOT/lib/discover-repos.sh" | sed 's/^/[found] /' || true
bash "$SSOT/sync-to-repos.sh"
bash "$SSOT/verify-sync.sh"
echo "=== done ==="
