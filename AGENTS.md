# AGENTS.md — map of this pack

## LAW vs MAP

- **Law:** `user-rules/USER-RULES.paste.txt` + `project-rules/*.mdc` + `hooks/` (+ live `.cursor/rules/`).
- **This file:** navigation only. Nested `AGENTS.md` wins for that subtree.

Before non-readonly work under a nested tree that has its own `AGENTS.md`, read that file.

## Overview

kleosrules — Cursor harness pack (Master Mind **V15.3**): User Rules paste, always-on companions, Rust `kleos-gate` + policy JSON (comments / secrets / vernacular / lean / ask-scope / pre-flight / gate-diff), personal skills, fleet sync. Single pack topology — not an app monorepo. **No Python in this pack.**

## Boundaries

| Boundary | Nested map | Owns |
|----------|------------|------|
| `hooks/` | [hooks/AGENTS.md](hooks/AGENTS.md) | kleos-gate + policy |
| `skills/` | [skills/AGENTS.md](skills/AGENTS.md) | On-demand Cursor skills |
| `docs/` | [docs/AGENTS.md](docs/AGENTS.md) | Doctrine + P* evals + TOOLCHAIN |
| `scripts/` | [scripts/AGENTS.md](scripts/AGENTS.md) | Install / scan / sync / verify / bench |
| `project-rules/` | [project-rules/AGENTS.md](project-rules/AGENTS.md) | Synced `.cursor/rules` sources |
| `user-rules/` | [user-rules/AGENTS.md](user-rules/AGENTS.md) | Paste + Option C mirror |
| `config/` | [config/AGENTS.md](config/AGENTS.md) | Skills list + scan roots |
| `lib/` | [lib/AGENTS.md](lib/AGENTS.md) | Discovery helpers (shell) |

## Where to look

| Need | Path | Notes |
|------|------|-------|
| Paste User Rules | `user-rules/USER-RULES.paste.txt` | V15.3 Option C |
| Option C disk mirror | `user-rules/option-c-core.mdc` | often `alwaysApply: true` |
| Always-on companions | `project-rules/{native-lean-autoload,ponytail,lean-code,agent}.mdc` | Synced |
| Vernacular contract | `.cursor/rules/vernacular.mdc` | Machine fields |
| Hot path binary | `hooks/bin/kleos-gate` | From `hooks/kleos-gate/` |
| Policy | `hooks/policy/*.json` | No hardcode in `.rs` |
| Hook registry | `hooks/hooks.json`, `hooks/hooks.project.json` | kleos-gate only |
| House verify | `cargo test -p kleos-gate` | + `benchmark-hooks.sh` |
| CLI tools | `kleos-gate gate-diff` / `obedience-report` / `check-user-rules` / `--check-content` | |
| Install to `~/.cursor` | `install.sh` / `hooks/install-user-hooks.sh` | |
| Fleet scan/sync | `scripts/scan-and-sync.sh` | |
| Sync verify | `scripts/verify-sync.sh` | |
| Release | `docs/RELEASE.md` | |
| TOOLCHAIN | `docs/TOOLCHAIN.md` | Done recipe |
| Pack version | `package.json` | `15.3.0` |

## Tree (depth 2)

```
.
├── install.sh
├── package.json
├── README.md
├── AGENTS.md
├── user-rules/
├── project-rules/
├── hooks/               # kleos-gate + policy + bin
├── skills/
├── config/
├── scripts/
├── lib/
├── docs/
└── .github/workflows/
```

## Done / verify

**Done = `docs/TOOLCHAIN.md` commands green with evidence.**

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
bash scripts/verify-sync.sh
bash scripts/benchmark-hooks.sh
hooks/bin/kleos-gate gate-diff
```

## Hard stops (Never)

- Never hand-edit downstream synced copies — edit this pack and re-sync.
- Never reintroduce Python into this pack (hooks, scripts, lib, proof).
- Never put secrets in paste rules, hooks JSON, scan roots, or chat dumps.
- Never treat green TOOLCHAIN as ∀ semantic quality (Rice).
- Never fight a live deny.

## Ask first

- Fleet `scan-and-sync` against new/untrusted roots
- Remote publish / force-push / hard reset / tree wipe
- Changing User Rules paste via MCP for other machines/users

## Deep links

- [docs/TOOLCHAIN.md](docs/TOOLCHAIN.md)
- [docs/RELEASE.md](docs/RELEASE.md)
- [docs/evals/HARDCODED-EXECUTION-SCHISM-PSTAR.md](docs/evals/HARDCODED-EXECUTION-SCHISM-PSTAR.md)
- [README.md](README.md)

## Manual notes

<!-- Human-owned landmines. agents-map preserves this section on refresh. -->
