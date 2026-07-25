# Closed-loop coupling — graph integrity

Breakthrough (2026-07-24): software is a **directed graph**, not a linear
essay. Isolated-file brilliance with missing edges is structural entropy
(orphans, unwired APIs, dead imports). Dilution of attention as context
grows is fatigue — not stupidity.

Companions: [`EPISTEMIC-RESONANCE.md`](EPISTEMIC-RESONANCE.md),
[`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md),
[`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md).

## Defect: linear-writing illusion

Models optimize for “file looks done.” Success metric must be **circuit
closed**: data flows A→B; every new node has edges; dead weight gone.

## Four directives

### A. Complete graph (no hanging wires)

Every new function, class, endpoint, or component must be imported/called
by at least one live path. If you write it, you wire it. Unconnected node
= task incomplete.

### B. Atomic state passes (anti-fatigue)

Non-trivial work in sequential passes; dump state to disk between passes.
Example order: data → domain/logic → UI → wiring. Do not thrash randomly
across ten open files. Per-pass max intelligence; no simultaneous whole-tree
retention.

### C. Flow tracing (before Done)

Before Done on a feature path: speak a short trace in chat —
user/trigger → entry → new code → response/side effect. Cognitive dry-run
of the graph; missing link → fix before Done.

### D. Negative entropy (delete the dead)

Same diff: grep callers; remove broken/unused imports and dead paths you
created or orphaned. No commented-out leftovers. Clean graph → less noise
→ less future fatigue.

### E. Epistemic persist (no black hole)

Non-local / temporal / concurrency constraints from Phase A must not die
with the chat. Encode in types/tests when possible; else durable O(1)
(`INVARIANTS.md`, ADR, DEBT, HANDOFF, package AGENTS) — never prose in
app source. See [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md).

## Success metric

Not “stopped generating tokens.” Circuit closed, edges traced, entropy
down or flat, fragile knowledge durable.

## Related

- Epistemic resonance: [`EPISTEMIC-RESONANCE.md`](EPISTEMIC-RESONANCE.md)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
