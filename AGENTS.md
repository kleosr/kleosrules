# AGENTS.md — map of this pack

## LAW vs MAP

- **Law:** `user-rules/USER-RULES.paste.txt` + `project-rules/*.mdc` + `hooks/` (+ live `.cursor/rules/`).
- **This file:** navigation only. Nested `AGENTS.md` wins for that subtree.

Before non-readonly work under a nested tree that has its own `AGENTS.md`, read that file.

## Overview

kleosr — Cursor harness pack (Master Mind **V16**): User Rules paste, always-on companions, Rust `kleos-gate` + policy JSON (comments / secrets / vernacular / lean / ask-scope / pre-flight / gate-diff) + fleet CLI (install / sync / verify / bench), MUST-NEVER M/J, soft=J authority (never waive M), lean size roofs ≠ ∀ quality, personal skills, fleet sync. Single pack topology — not an app monorepo. **No Python. No pack shell — tooling is Rust-only.**

## Boundaries

| Boundary | Nested map | Owns |
|----------|------------|------|
| `hooks/` | [hooks/AGENTS.md](hooks/AGENTS.md) | kleos-gate (engine + fleet) + policy |
| `skills/` | [skills/AGENTS.md](skills/AGENTS.md) | On-demand Cursor skills |
| `docs/` | [docs/AGENTS.md](docs/AGENTS.md) | Doctrine + P* evals + TOOLCHAIN |
| `project-rules/` | [project-rules/AGENTS.md](project-rules/AGENTS.md) | Synced `.cursor/rules` sources |
| `user-rules/` | [user-rules/AGENTS.md](user-rules/AGENTS.md) | Paste + Option C mirror |
| `config/` | [config/AGENTS.md](config/AGENTS.md) | Skills list + scan roots |

## Where to look

| Need | Path | Notes |
|------|------|-------|
| Paste User Rules | `user-rules/USER-RULES.paste.txt` | V15.6 Option C (M/J + soft J) |
| Option C disk mirror | `user-rules/option-c-core.mdc` | often `alwaysApply: true` |
| Always-on companions | `project-rules/{native-lean-autoload,ponytail,lean-code,agent}.mdc` | Synced |
| Vernacular contract | `project-rules/vernacular.mdc` | Pack SSOT; live link under `.cursor/rules/` |
| Hot path + fleet CLI | `hooks/bin/kleos-gate` | From `hooks/kleos-gate/` |
| Policy | `hooks/policy/*.json` | No hardcode in `.rs` |
| Hook registry | `hooks/hooks.json`, `hooks/hooks.project.json` | kleos-gate only |
| House verify | `cargo test -p kleos-gate` | + `kleos-gate bench` |
| CLI tools | `install` / `sync` / `verify` / `bench` / `gate-diff` / `obedience-report` / `check-user-rules` / `--check-content` | |
| Install to `~/.cursor` | `kleos-gate install` | |
| Fleet scan/sync | `kleos-gate sync` | |
| Sync verify | `kleos-gate verify` | |
| Release | `docs/RELEASE.md` | |
| TOOLCHAIN | `docs/TOOLCHAIN.md` | Done recipe |
| Pack version | `package.json` | `16.0.0` (kleosr) |
| Pack vernacular SSOT | `project-rules/vernacular.mdc` | Linked into pack `.cursor/rules` |
| P*-13 M/J | `docs/evals/PERFORMATIVE-TRILEMMA-PSTAR.md` | Language kill |
| P*-14 soft-force | `docs/evals/SOFT-FORCE-SCHISM-PSTAR.md` | Skill Self-target |
| P*-15 lean≠quality | `docs/evals/LEAN-SIZE-QUALITY-PSTAR.md` | Size roofs ≠ YAGNI |

## Tree (depth 2)

```
.
├── AGENTS.md
├── README.md
├── LICENSE
├── package.json
├── user-rules/
├── project-rules/
├── hooks/               # kleos-gate (engine + fleet) + policy + bin
├── skills/
├── config/
├── docs/
└── .github/            # CI only; not enforcement
```

`.cursor/` is local/sync dest (gitignored) — not SSOT. Enforcement = `hooks/policy/*.json` + `hooks/bin/kleos-gate` only.

Hunt timeline: [`docs/RULES-HUNT.md`](docs/RULES-HUNT.md).

## Done / verify

**Done = `docs/TOOLCHAIN.md` commands green with evidence.**

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
hooks/bin/kleos-gate verify
hooks/bin/kleos-gate bench
hooks/bin/kleos-gate gate-diff
```

## Hard stops (Never)

- Never hand-edit downstream synced copies — edit this pack and re-sync.
- Never reintroduce Python or pack `.sh` tooling into this pack.
- Never put secrets in paste rules, hooks JSON, scan roots, or chat dumps.
- Never treat green TOOLCHAIN as ∀ semantic quality (Rice).
- Never fight a live deny.

## Ask first

- Fleet `sync` against new/untrusted roots
- Remote publish / force-push / hard reset / tree wipe
- Changing User Rules paste via MCP for other machines/users

## Deep links

- [docs/TOOLCHAIN.md](docs/TOOLCHAIN.md)
- [docs/RELEASE.md](docs/RELEASE.md)
- [docs/evals/HARDCODED-EXECUTION-SCHISM-PSTAR.md](docs/evals/HARDCODED-EXECUTION-SCHISM-PSTAR.md)
- [README.md](README.md)

## Manual notes

<!-- Human-owned landmines. agents-map preserves this section on refresh. -->
