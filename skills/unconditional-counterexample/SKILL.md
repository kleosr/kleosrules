---
name: unconditional-counterexample
description: >
  Runs a fixed 4-phase hunt that delivers one complete unconditional
  counterexample without the user re-prompting between phases. Use when the
  user invokes /unconditional-counterexample, pastes the counterexample
  mantra, says "fully works", "breakthrough", "partial results", or wants a
  structured counterexample to a general claim/conjecture/rules/product.
---

# Unconditional Counterexample

## Auto-pipeline (do not make the user re-prompt)

On invoke, fill `{problem}` from the user message (claim, conjecture, product, or rules under attack). Then run **Phases 1→2→3→4 in order in this same turn-loop**. Do not stop after Phase 1 or 2 to wait for the user to paste the next block. Advance automatically until Phase 4’s done-when is met (or a hard blocker: missing target text / no readable sources).

Track progress mentally:

```
- [ ] Phase 1 — structured breakthrough
- [ ] Phase 2 — complete unconditional
- [ ] Phase 3 — strategy from structure
- [ ] Phase 4 — finish (no partials)
```

---

## Phase 1 — breakthrough (verbatim shape)

Construct a counterexample to general ({problem}). You should do a breakthrough and find a structured counterexample.

(Archetype fill from the user’s screenshot pattern: `non-planar case of Dinitz Garg Goemans conjecture` — replace with whatever `{problem}` they named.)

**Agent does:** Restate \(C\). Map structure. Produce a first structured candidate \(P\) (or prove why none yet). Do **not** end the run here.

---

## Phase 2 — continue (verbatim)

please continue research and find a complete unconditional counterexample

**Agent does:** Deepen evidence. Kill weak/partial candidates. Push toward complete + unconditional. Continue immediately into Phase 3 if still incomplete.

---

## Phase 3 — strategy (verbatim)

Continue the search. Have a clear strategy obtained from deeper understanding of the problem structure.

**Agent does:** State the structural strategy in 1–3 sentences, then use it to search again (writers/readers, dropped fields, dual surfaces, conjunction/precedence holes, etc.). Do not stop at “here is a strategy.”

---

## Phase 4 — finish (verbatim)

it's enough of partial results. let's finish with a complete unconditional counterexample

**Agent does:** Deliver **one** finished \(P^*\). No ranked maybe-list as the answer.

---

## Done-when (Phase 4 only)

| Property | Meaning |
|----------|---------|
| **Complete** | Claim \(C\), instance \(P^*\), failure mode, file:line or clause evidence, why \(\neg C\) |
| **Unconditional** | Fails by construction — no race, flake, network, token expiry, or “sometimes” |

Sibling gaps only **after** \(P^*\). They do not replace \(P^*\).

## Hunt checklist

- Split brain (two writers / reader ignores a write)
- Dropped field (producer has \(X\); consumer hardcodes default)
- Dead claimed path (exists, never used on advertised flow)
- Missing gate (upstream checks; this surface skips)
- Conjunction / precedence / ACT-vs-gated label clashes (rules targets)

## Output (emit once at Phase 4)

```markdown
## Verdict
[one line]

## Strategy
[1–3 sentences from Phase 3]

## Claim (C)
…

## Instance (P*)
1. …
2. …
3. …

## Failure (by construction)
- Evidence: …
- What always happens
- Why this falsifies C

## Sibling gaps (optional)
| Gap | Why weaker |
```

## Anti-patterns

- Stopping after Phase 1–3 for the user to paste the next prompt
- Ending on partial concerns with no \(P^*\)
- Flaky live-only “might fail”
- Fixing during the hunt unless asked

## After delivery

Fix plan only if the user asks. Kill \(P^*\) only; no sibling-gap expansion unless asked.
