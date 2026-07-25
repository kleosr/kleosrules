# P* — Precedence paradox (MUST-NEVER vs gates)

Finished unconditional counterexample. Closed in V10.1.5.

## Verdict

V10.1 text called MUST-NEVER a soft rule needing a gate, and said gates
outrank soft policy on gated surfaces, while also saying MUST-NEVER first
and Nothing waives MUST-NEVER. Those axioms were mutually inconsistent.

## Claim (C)

The precedence stack is coherent: MUST-NEVER is inviolable, and mechanical
gates outrank soft policy on gated surfaces, with MUST-NEVER treated as soft
policy that must live behind a gate.

## Instance (P*)

1. MECHANICAL GATES (pre-kill): soft without gate = advisory; if MUST-NEVER
   named, must have live gate (reads as MUST-NEVER ∈ soft).
2. PRECEDENCE: MUST-NEVER first; gates outrank soft policy on gated surfaces.
3. MUST-NEVER: Nothing waives MUST-NEVER.

| Branch | Violates |
|--------|----------|
| Gates outrank MUST-NEVER (B∧C) | MUST-NEVER first / Nothing waives |
| MUST-NEVER outranks gates (A) | Gates outrank soft on gated surfaces if MUST-NEVER is soft |

## Failure (by construction)

Contradiction in contract text alone — no runtime, parser, or crash required.

## Kill (V10.1.5)

- Soft rules = non-MUST-NEVER only.
- MUST-NEVER / named ASK = roof; gates implement, never outrank.
- Gate allow ≠ waiver; refuse / fix / ask on conflict.
- PRECEDENCE: MUST-NEVER first including over gates; gates outrank soft
  (non-MUST-NEVER) only.

Sibling (weaker): Rice / semantic undecidability of perfect syntactic gates —
orthogonal; does not reopen the hierarchy paradox once roof ≠ soft.
