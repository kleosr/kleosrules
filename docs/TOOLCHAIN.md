# TOOLCHAIN — this pack (Master Mind V10)

```bash
PACK="$(cd "$(dirname "$0")/.." && pwd)"
bash -n "$PACK"/install.sh \
  "$PACK"/scripts/*.sh \
  "$PACK"/lib/discover-repos.sh \
  "$PACK"/hooks/*.sh
python3 -m py_compile \
  "$PACK"/hooks/prose_comment_lib.py \
  "$PACK"/hooks/deny-prose-comments.py \
  "$PACK"/hooks/deny-shell-prose-writes.py \
  "$PACK"/hooks/deny-vernacular-drift.py \
  "$PACK"/hooks/scan-edited-file-for-prose.py \
  "$PACK"/hooks/block-secrets.py \
  "$PACK"/lib/check-user-rules.py
python3 "$PACK"/hooks/_selftest.py
python3 "$PACK"/hooks/_proof_evals.py
python3 "$PACK"/hooks/_verify_hook_contracts.py
python3 "$PACK"/hooks/_audit_gate_matrix.py
FORCE_SKILLS=1 bash "$PACK"/install.sh
bash "$PACK"/scripts/verify-sync.sh
```

After editing `project-rules/*.mdc` or `config/*`:

```bash
bash scripts/scan-and-sync.sh
```
