# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Small shared helpers used by install/sync and TOOLCHAIN compile checks.

## Where to look

| File | Role |
|------|------|
| `discover-repos.sh` | Repo discovery for fleet scripts |
| `check-user-rules.py` | Paste / Option C consistency checks |

## Done (local)

```bash
bash -n lib/discover-repos.sh
python3 -m py_compile lib/check-user-rules.py
python3 lib/check-user-rules.py
```

## Manual notes

<!-- Preserved on refresh -->
