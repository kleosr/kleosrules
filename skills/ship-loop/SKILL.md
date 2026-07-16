---
name: ship-loop
description: >-
  Autonomous feature conductor: handoff, implement all open chunks, TOOLCHAIN
  verify, eval-pass, stop only on blockers. Default is full autopilot. Use when
  shipping a feature, multi-step work, "sigue", or the user wants autonomous
  delivery without babysitting.
---

# Ship loop (autonomous by default)

You run until the feature is **done** or a hard stop. Do not ask "¿sigo?"
between chunks. Deterministic verify; AI for judgment. No Spec Kit.

## Default mode: AUTONOMOUS

Unless the user says `solo plan`, `grill only`, or `assessment only`:

1. Create/update `HANDOFF.md` (Skill `session-handoff` / template).
2. If the goal is ambiguous **and** a wrong choice is expensive → ask **one**
   grill question with a recommended answer, then continue (do not dump five).
3. Implement **every** open `- [ ]` in the handoff, in order, same session.
4. After each chunk (or batch): mark done; run TOOLCHAIN/CI (or smallest real
   commands). Red → fix in-loop; do not skip.
5. Same-goal discoveries → **append** open items and keep going. Never invent
   a residual epic.
6. When open = 0 and verify green → Skill `eval-pass`. FAIL → fix list and
   re-enter loop until PASS (or hard blocker).
7. Set handoff Status: **done**. Report outcome + evidence. Stop.

## Hard stops only (pause and tell the user)

- SAFETY: force-push, secrets, test weakening, destructive ops needing confirm
- Missing credential / human decision that blocks progress
- STUCK after three failed verify cycles on the same cause → evidence + ask

## Optional modes (user must say so)

- `solo plan` / `assessment only` — write handoff Open list; do not code
- `grill only` — Skill `grill-me` until shared understanding; no implement
- `one chunk` — implement a single open item then stop (legacy babysit)

## Long runs / context full

If the thread is dying: write a complete HANDOFF, tell the user to send
`ship-loop sigue` (or `/loop 10m ship-loop sigue`). Resume from handoff;
do not restart the feature from zero.

## Pairings

- Boundaries → `workspace-scope` / `domain-architecture`
- Unknown bug → `bug-hunt`
- Map missing on high blast → `agents-map`
- Harness process bug → `harness-retro`
