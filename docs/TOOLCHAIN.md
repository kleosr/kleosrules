# TOOLCHAIN — this pack

```bash
PACK="$(cd "$(dirname "$0")/.." && pwd)"
bash -n "$PACK"/install.sh \
  "$PACK"/scripts/*.sh \
  "$PACK"/lib/discover-repos.sh \
  "$PACK"/hooks/*.sh
python3 -m py_compile \
  "$PACK"/hooks/deny-prose-comments.py \
  "$PACK"/hooks/block-secrets.py \
  "$PACK"/lib/check-user-rules.py
printf '%s' '{"input":{"path":"a.ts","contents":"x=1\n// bad\n"}}' \
  | python3 "$PACK"/hooks/deny-prose-comments.py | grep -q deny
FORCE_SKILLS=1 bash "$PACK"/install.sh
bash "$PACK"/scripts/verify-sync.sh
```

After editing `project-rules/*.mdc` or `config/*`:

```bash
bash scripts/scan-and-sync.sh
```
