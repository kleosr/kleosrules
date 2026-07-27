# AGENTS.md — map of this pack

## LAW vs MAP

- **Law:** `user-rules/USER-RULES.paste.txt` + `project-rules/*.mdc` + `hooks/` (+ live `.cursor/rules/`).
- **This file:** navigation only. Nested `AGENTS.md` wins for that subtree.

Before non-readonly work under a nested tree that has its own `AGENTS.md`, read that file.

## Overview

kleosrules — Cursor harness pack (Master Mind **V15.1**): User Rules paste, always-on companions, Rust `kleos-gate` + policy JSON (comments / secrets / vernacular / lean / ask-scope), personal skills, fleet sync. Single pack topology — not an app monorepo.

## Boundaries

| Boundary | Nested map | Owns |
|----------|------------|------|
| `hooks/` | [hooks/AGENTS.md](hooks/AGENTS.md) | kleos-gate + policy |
| `skills/` | [skills/AGENTS.md](skills/AGENTS.md) | On-demand Cursor skills |
| `docs/` | [docs/AGENTS.md](docs/AGENTS.md) | Doctrine + P* evals + TOOLCHAIN |
| `scripts/` | [scripts/AGENTS.md](scripts/AGENTS.md) | Install / scan / sync / verify |
| `project-rules/` | [project-rules/AGENTS.md](project-rules/AGENTS.md) | Synced `.cursor/rules` sources |
| `user-rules/` | [user-rules/AGENTS.md](user-rules/AGENTS.md) | Paste + Option C mirror |
| `config/` | [config/AGENTS.md](config/AGENTS.md) | Skills list + scan roots |
| `lib/` | [lib/AGENTS.md](lib/AGENTS.md) | Discovery / paste check helpers |

## Where to look

| Need | Path | Notes |
|------|------|-------|
| Paste User Rules | `user-rules/USER-RULES.paste.txt` | V15.1 Option C |
| Option C disk mirror | `user-rules/option-c-core.mdc` | `alwaysApply: false` |
| Always-on companions | `project-rules/{native-lean-autoload,ponytail,lean-code,agent}.mdc` | Synced |
| Vernacular contract | `.cursor/rules/vernacular.mdc` | Machine fields |
| Hot path binary | `hooks/bin/kleos-gate` | From `hooks/kleos-gate/` |
| Policy | `hooks/policy/*.json` | No hardcode in `.rs` |
| Hook registry | `hooks/hooks.json`, `hooks/hooks.project.json` | No python3 |
| House verify | `cargo test -p kleos-gate` | `tests/integration.rs` |
| Install to `~/.cursor` | `install.sh` / `hooks/install-user-hooks.sh` | |
| Fleet scan/sync | `scripts/scan-and-sync.sh` | |
| Sync verify | `scripts/verify-sync.sh` | |
| Release | `docs/RELEASE.md` | |
| TOOLCHAIN | `docs/TOOLCHAIN.md` | Done recipe |
| Pack version | `package.json` | `15.1.0` |

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
```

## Hard stops (Never)

- Never hand-edit downstream synced copies — edit this pack and re-sync.
- Never reintroduce `python3` on Cursor hook hot path or house proof.
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
- [docs/evals/ANTI-DRIFT-DRIVE-BY-PSTAR.md](docs/evals/ANTI-DRIFT-DRIVE-BY-PSTAR.md)
- [README.md](README.md)

## Manual notes

<!-- Human-owned landmines. agents-map preserves this section on refresh. -->
