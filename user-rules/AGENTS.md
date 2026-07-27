# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Master Mind Option C paste body and disk mirror for Cursor User Rules.

## Where to look

| File | Role |
|------|------|
| `USER-RULES.paste.txt` | Canonical paste → Cursor Settings → User Rules (V14) |
| `option-c-core.mdc` | Disk mirror; `alwaysApply: false` — single source with paste |

## Done (local)

```bash
python3 lib/check-user-rules.py
```

After paste text changes: inject/update Cursor User Rules (MCP or manual paste), then **new chat** so the agent reloads rules.

## Hard stops (this package)

- Never run both full User Rules paste **and** `option-c-core` as alwaysApply true (duplicate constitution).
- Never put secrets in the paste body.

## Ask first

- MCP/bulk overwrite of User Rules on shared machines

## Manual notes

<!-- Preserved on refresh -->
