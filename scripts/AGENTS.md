# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Fleet install/sync/verify. Mutates other repos’ `.cursor` when scan roots are set.

## Where to look

| Task | Location | Notes |
|------|----------|-------|
| Discover + sync rules/hooks/skills | `scan-and-sync.sh` | Reads `config/scan.roots` |
| Sync hooks only | `sync-hooks-to-repos.sh` | `kleos-gate` binary + policy |
| Sync rules/skills plane | `sync-to-repos.sh` | Companion plane |
| Verify fingerprints | `verify-sync.sh` | HOOK_NEED + companions |
| Pre-commit install helper | `install-pre-commit.sh` | `kleos-gate gate-diff` |
| Obedience report | `kleos-gate obedience-report` | Rust CLI |
| Hook adversarial bench | `benchmark-hooks.sh` | Live deny/ask/allow + `--check-content` |

## Done (local)

```bash
bash -n scripts/*.sh
bash scripts/verify-sync.sh
bash scripts/benchmark-hooks.sh
```

Full house: [`docs/TOOLCHAIN.md`](../docs/TOOLCHAIN.md).

## Hard stops (this package)

- Never point `scan.roots` at untrusted trees without user confirmation.
- Never reintroduce Python hook or proof files into this pack.

## Ask first

- Expanding `config/scan.roots` / clearing `scan.ignore`
- Running sync against production app repos mid-feature

## Manual notes

<!-- Preserved on refresh -->
