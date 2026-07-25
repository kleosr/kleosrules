# P* — Local conjunction ≠ global correctness

Finished unconditional counterexample (composition gap). Named in V10.1.15.

## Verdict

A system can obey every local protocol perfectly and still be globally wrong.
The failure is produced by *following* the rules, not by breaking them.

## Claim (C)

If each atomic pass / node / contract verifies locally (closed-loop wired,
contracts green, persist landed), the composed system is correct.

## Instance (P*)

Timezone/seam class: Pass data accepts a date string; Pass logic stores
“the same string”; Pass UI formats “the same string”; each contract
“round-trip equals input” is green. Embodied path: Yuki in Tokyo books
23:00 March 5 → confirmation shows March 6. Every local check passed;
the relation between layers (implicit local vs UTC) failed.

## Failure (by construction)

`P₁ ∧ P₂ ∧ P₃` never implies global correctness. Error habitat is seams and
trajectories, not nodes. No model mistake required — perfect protocol
obedience still yields the bug.

## Kill (incomplete by design — chain)

1. Name the gap; distrust “total unconditional” closure claims
   (`docs/VERIFICATION-CHAIN.md`).
2. Attack relations: [`RELATIONAL-VERIFICATION.md`](../RELATIONAL-VERIFICATION.md).
3. Accept residual unimagined trajectories; close with production:
   [`REALITY-LOOP.md`](../REALITY-LOOP.md).

Mature engineering does not eliminate the gap; it names it (invariants /
executable seams) and refuses omniscience theater.
