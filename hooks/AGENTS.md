# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Mechanical Cursor hooks: Rust `kleos-gate` binary + `policy/*.json`, install into
`~/.cursor/hooks`, house verify via `cargo test`. Fleet ops live in the same binary.

## Layer map

| Proposed | Actual |
|----------|--------|
| `gate/` | `kleos-gate/src/engine/` |
| `cli/` | `kleos-gate/src/fleet/` + `main.rs` |
| `policy/` | `kleos-gate/src/policy.rs` + `policy/*.json` |
| `tests/` | `kleos-gate/tests/{common,integration,lean_meter,vernacular,token_patterns,soft_force_no_waiver,cli_reports}.rs` |

## Where to look

| Task | Location | Notes |
|------|----------|-------|
| Hot path binary | `bin/kleos-gate` | Regenerable; gitignored — build via TOOLCHAIN |
| Engine source | `kleos-gate/src/engine/` | write, shell, read, mcp, … |
| Fleet CLI | `kleos-gate/src/fleet/` | install, sync, verify, bench, discover |
| Policy (no hardcode) | `policy/*.json` | shell, lean, secrets, ask-scope, delete |
| Registries | `hooks.json`, `hooks.project.json` | User vs project |
| Install to home | `kleos-gate install` / `install-hooks` | Rust CLI |
| CLI tools | `kleos-gate` args | `--check-content`, `gate-diff`, `obedience-report`, `check-user-rules`, `sync`, `verify`, `bench` |
| Verify | `cd kleos-gate && cargo test` | Named M suites under `tests/` |

## Done (local)

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
cp -f kleos-gate/target/release/kleos-gate bin/kleos-gate
hooks/bin/kleos-gate verify
```

See root [`docs/TOOLCHAIN.md`](../docs/TOOLCHAIN.md).

## Hard stops (this package)

- Missing/invalid policy → deny.
- Never reintroduce Python or pack shell into this pack (hooks, scripts, lib, proof).
- Never embed shell/lean/secrets regex policy in `.rs`.
- Never commit `bin/kleos-gate` (regenerable).

## Ask first

- Changing deny → allow on destructive / remote-publish classes
- Shipping without rebuild + verify to live `~/.cursor`

## Manual notes

<!-- Preserved on refresh -->
