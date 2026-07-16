# AGENTS.md — map only (Documentos/rules)

## LAW vs MAP

- **Law:** root `*.mdc` (loaded via `.cursor/rules` symlinks).
- **This file:** map of the harness only.

## Overview

SSOT harness for Cursor project rules and owned personal Skills. Rules fan
out to **discovered** projects; Skills are symlinked globally into
`~/.cursor/skills/`.

## Where to look

| Task | Location |
|------|----------|
| Always-on law | `agent.mdc` |
| Types / testing / debugging | `types.mdc`, `testing.mdc`, `debugging.mdc` |
| User paste layer | `USER-RULES.paste.txt` / `USER-RULES.md` (below `---`) |
| Discovery config | `scan.roots`, `scan.ignore`, `lib/discover-repos.sh` |
| One-shot scan+sync | `scan-and-sync.sh` |
| Managed Skills / list | `skills/`, `skills.txt` |
| User SAFETY hooks SSOT | `hooks/` (+ `install-user-hooks.sh`) |
| Retired rule names | `retired.txt` |
| Retired Skill names | `retired-skills.txt` |
| Done for this repo | `TOOLCHAIN.md` |
| Layer design | `README.md` |
| Personal Skill runtime links | `~/.cursor/skills/` |
| Deprecated fleet file | `repos.txt` (comment-only; do not use) |

## Done / verify

See `TOOLCHAIN.md`. After any `*.mdc` edit: `scan-and-sync.sh` (or sync + verify).

## Hard stops (Never)

- Never hand-edit `.cursor/rules` copies under owned repos; edit SSOT and sync.
- Never replace SSOT symlinks under `rules/.cursor/rules/` with divergent files.
- Never hand-edit managed Skills through `~/.cursor/skills/`; edit `skills/`.
- Never add a 5th always-on craft `.mdc` without retiring something or moving
  content to a Skill / TOOLCHAIN (see README).
- Never reintroduce `/home/kleosr/Descargas/Reglas` as runtime rules.

## Ask first

- Changing SAFETY semantics in `agent.mdc`
- Adding scan roots outside Documentos (blast radius)

## Manual notes

<!-- Human-owned -->
