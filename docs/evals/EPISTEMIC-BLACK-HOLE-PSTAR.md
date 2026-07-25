# P* — Epistemic black hole (silent code vs durable knowledge)

Finished unconditional counterexample. Closed in V10.1.14.

## Verdict

Silent application source without durable O(1) anchors for non-local
invariants falsifies “zero drift forever” while chat blueprints evaporate.

## Strategy

Split surfaces: ban prose in app code (roof) but require persistence of
non-local / temporal / concurrency constraints on durable repo surfaces
or in types/tests — never leave them only in Session-1 chat.

## Claim (C)

Silent transmutation + NO prose comments + Phase-A chat blueprints
together guarantee maximum quality and zero attentional drift across
sessions and agents on growing repos.

## Instance (P*)

1. Session 1: agent ships a non-trivial concurrency / recursive state /
   conditional-type subsystem under NO COMMENTS + silent Phase B; Done.
2. Session 2 (weeks later): new reader must obey Read-first; no comments;
   chat blueprint gone.
3. Non-local invariant (e.g. “caller must hold DB lock; do not call from
   unbound async without ExecutionContext”) is not expressible as a local
   name or local type alone.

| Escape | Why it fails by construction |
|--------|------------------------------|
| Names/types carry it | Types shape data; not temporal/distributed caller contracts |
| Chat held the blueprint | Chat is ephemeral; repo has no Session-1 CoT |
| AGENTS.md alone | High-altitude map; micro-fragile edges live below map grain |
| “Just read all the code” | O(N) reconstruct → Lost-in-the-Middle → drift Session 2 |

## Failure (by construction)

Syntax comprehension of architecture is O(N). Durable prose/typed/test
anchors are O(1). Forbidding *all* durable anchors while discarding chat
guarantees future context saturation. ¬C on any repo past trivial size —
no race, flake, or model IQ required.

## Kill (V10.1.14)

- Silent **code** ≠ silent **repo**.
- Prefer encode invariants in types, required context params, asserts,
  and tests named as the invariant.
- Else persist O(1) on durable surfaces: module `INVARIANTS.md`, ADR /
  `docs/`, `DEBT.md`, `HANDOFF.md`, or a package AGENTS subsection —
  **never** prose comments in application source.
- Before Done: Phase-A constraints that would be fragile if forgotten →
  SYSTEM INTEGRITY **EPISTEMIC PERSIST** (incomplete until landed).
- Writing that anchor is closed-loop integrity, not “architecture theater.”

Sibling (weaker): perfect encoding of all informal English into types is
undecidable in general — orthogonal; does not reopen the need for *some*
durable O(1) surface when types/tests cannot carry the load.
