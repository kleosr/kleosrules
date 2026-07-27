# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Mechanical Cursor hooks: Rust `kleos-gate` binary + `policy/*.json`, install into
`~/.cursor/hooks`, house verify via `cargo test`.

## Where to look

| Task | Location | Notes |
|------|----------|-------|
| Hot path binary | `bin/kleos-gate` | Built from `kleos-gate/` |
| Engine source | `kleos-gate/src/` | write, shell, read, mcp, … |
| Policy (no hardcode) | `policy/*.json` | shell, lean, secrets, ask-scope |
| Registries | `hooks.json`, `hooks.project.json` | User vs project |
| Install to home | `install-user-hooks.sh` | cargo build + copy |
| CLI tools | `kleos-gate` args | `--check-content`, `gate-diff`, `obedience-report`, `check-user-rules` |
| Verify | `cd kleos-gate && cargo test` | Integration meters |

## Done (local)

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
cp -f kleos-gate/target/release/kleos-gate bin/kleos-gate
```

See root [`docs/TOOLCHAIN.md`](../docs/TOOLCHAIN.md).

## Hard stops (this package)

- Missing/invalid policy → deny.
- Never reintroduce Python into this pack (hooks, scripts, lib, proof).
- Never embed shell/lean/secrets regex policy in `.rs`.

## Ask first

- Changing deny → allow on destructive / remote-publish classes
- Shipping without rebuild + verify-sync to live `~/.cursor`

## Manual notes

<!-- Preserved on refresh -->
