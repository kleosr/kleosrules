# Topological prompt — U-curve + recurrence anchors

Breakthrough (2026-07-24): attention over long rules is **U-shaped**
(primacy + recency; dead zone in the middle — “Lost in the Middle”,
Liu et al.). Putting every critical rule only at the start *or* only at
the end is a false choice.

Companion: [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md),
[`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md).

## U-curve layout (User Rules)

| Block | Position | Role | Content class |
|-------|----------|------|----------------|
| **1 Primacy** | Head | Who the agent is; absolute roof | IDENTITY, PRIME, MUST-NEVER, PRECEDENCE, ACT/ASK, CONFIRMATION |
| **2 Mid** | Middle | Passive dictionary / recovery | Companions list, vernacular, skills, gauntlet policy, update/single-source, doc pointers |
| **3 Recency** | Tail | Next-token execution gates | NO COMMENTS, NATIVE LEAN, MECHANICAL GATES, LOOP (+ SYSTEM INTEGRITY / closed-loop), VOICE |

Structural tokens (`---`, ALL-CAPS heads, numbered lists) create attention
peaks. Do not park roof or write-time gates in the mid dead zone alone.

## Recurrence anchors (anti-drift after long reads)

Distance from the original rules grows as the agent Reads files. Hooks
re-surface execution constraints at action time:

- `preToolUse` (Write|StrReplace|EditNotebook) — zero-comment / secrets /
  vernacular before land
- `beforeShellExecution` — danger / ask / shell-prose

That is simulated recurrence: attention recalculated at the write gate,
not only at chat open. Prefer hook re-anchor over trusting mid-prompt recall
after five file reads.

## Equation

1. Primacy initializes the safety bubble.
2. Mid holds passive lookup — not the only copy of write gates.
3. Recency + hooks force execution filters on the next token / next write.

## Related

- User Rules paste: `user-rules/USER-RULES.paste.txt` (topology applied)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
