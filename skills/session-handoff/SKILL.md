---
name: session-handoff
description: >-
  Write or resume a structured HANDOFF.md for multi-session agent work.
  Use when continuing tomorrow, switching chats/models, pausing a feature,
  or the user mentions handoff, HANDOFF, or session continuity.
---

# Session handoff

Fresh context beats a rotten 12-hour chat. Persist state in the **repo**,
not only in conversation.

## Paths

Prefer one active handoff:

1. `HANDOFF.md` at repo root (default), or
2. `.cursor/handoff/<slug>.md` when several features run in parallel

Template: `skills/session-handoff/references/handoff-template.md`
(SSOT under Documentos/rules).

## Resume

1. Read the handoff file if it exists.
2. Re-run the listed TOOLCHAIN/CI commands before claiming prior work is green.
3. Do **one** Next action unless the user said autopilot/`sigue`.
4. Do not re-discover the whole codebase; open only paths listed or needed.

## Pause / update

Rewrite the handoff with:

- Goal (one sentence)
- Done (paths + evidence commands actually run)
- Open (checkbox list; same goal only — append, do not spawn a new epic)
- Blockers
- Next (single concrete action)
- Model note (optional: which model owned this thread)

## Rules

- Assessment-only until the user asked for changes (per agent.mdc).
- Never invent residual features; append open items to this handoff.
- Pair with `ship-loop` for feature execution; `grill-me` if Open is ambiguous.
