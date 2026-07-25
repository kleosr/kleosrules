# Relational verification — seams and trajectories

Layer above local SYSTEM INTEGRITY. Kills the immediate class of
[`evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md`](evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md).
Companion: [`REALITY-LOOP.md`](REALITY-LOOP.md), [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md).

## Object shift

Verify **trajectories** (one concrete datum end-to-end), not only artifacts.
Bugs invisible per layer are trivial on an embodied path.

## Four mechanisms

### 1. Embodied traces first

Before non-trivial Pass 1: 3–5 hostile personas with **literal** inputs and
**hand-computed** expected outputs (immutable). Passengers for the finished
system — not unit stubs alone.

### 2. Executable invariants

Global conventions (TZ, units, encoding) = boundary assertions or distinct
types that refuse silent ambiguous crossing. Prefer fail-loud at the seam
over prose in `INVARIANTS.md` alone.

### 3. Differential oracle

Derive expected results **without reading the implementation**, in a
separate prior pass; compare to system output. Breaks shared-blind-spot
correlation between author and verifier (not absolute certainty).

### 4. Seam pass

After wiring: enumerate each boundary (UI→API, API→domain, domain→store).
For each, with **executed** evidence: exact format crossing; both sides
parse alike; hostile persona #N end-to-end.

## Protocol (mandatory above local protocols)

```
RELATIONAL VERIFICATION
1. EMBODIED TRACES FIRST — 3–5 concrete hostile personas + hand oracles.
2. EXECUTABLE INVARIANTS — boundary asserts / distinct types; no silent seams.
3. DIFFERENTIAL ORACLE — expect before reading impl; compare.
4. SEAM PASS — after wiring, audit intersections with terminal evidence.
```

## Honesty

Does not eliminate every unimagined trajectory. Shrinks work from infinite
composition space to finite hostile personas + seams. Residual → reality-loop.

## Related

- Chain: [`VERIFICATION-CHAIN.md`](VERIFICATION-CHAIN.md)
- Closed-loop: [`CLOSED-LOOP-COUPLING.md`](CLOSED-LOOP-COUPLING.md)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
