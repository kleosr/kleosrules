# AGENTS.md — map only

## LAW vs MAP

- **Law:** `.cursor/rules/*.mdc` (synced from this rules pack). Do not restate SAFETY/QUALITY here.
- **This file:** map of *this* repo only. Nested `AGENTS.md` under package boundaries win for that subtree.

Before non-readonly work under a nested tree that has its own `AGENTS.md`, read that file.

## Overview

<!-- 2–4 lines: product, stack one-liner, workspace topology if relevant -->

## Where to look

| Task | Location | Notes |
|------|----------|-------|
| <!-- e.g. HTTP entry --> | <!-- path --> | <!-- entry / pattern --> |
| | | |

## Done / verify

- If `TOOLCHAIN.md` exists: **Done = those commands green with evidence in the report.**
- If missing: say so; run only commands proven from package scripts/CI; flag the gap.

See: `TOOLCHAIN.md` (or "missing — create via agents-map").

## Hard stops (Never)

<!-- 3–8 bullets. Only non-discoverable or high-blast risks evidenced in-repo.
     Delete entire section if none. Examples of shape, not content to invent:
- Never log/commit PII or chat bodies.
- Never auto-approve moderated entities in production paths.
- Never dump session vaults / cookies / stealth profiles into chat or git.
- Never force dual Discord gateways on one token.
-->

-

## Ask first

<!-- Ops with blast radius. User must explicitly request in-session. -->

- Migrations / destructive DB / data backfills
- Deploy, cluster apply, DNS/TLS, or committing secret material
- Live browser/provider automation against real accounts
- Bulk delete/clear of operational or user data

## Deep links

- <!-- existing docs only -->
- <!-- DEBT.md if present -->

## Manual notes

<!-- Human-owned landmines. agents-map preserves this section on refresh. -->
