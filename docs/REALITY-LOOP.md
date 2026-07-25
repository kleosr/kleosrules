# Reality-loop — post-contact falsification

Wraps pre-deploy protocols. After relational verification’s residual
(“the trajectory nobody embodied”), correctness is a **maintained process**,
not a once-certified state. Design for failures that are cheap, visible,
reversible.

Companions: [`RELATIONAL-VERIFICATION.md`](RELATIONAL-VERIFICATION.md),
[`evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md`](evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md),
[`AGENTIC-GAUNTLET.md`](AGENTIC-GAUNTLET.md).

## Paradigm

Pre-deploy checks are imagined falsification. Production traffic is the only
oracle that shares no author blind spot. Deploy = densest information phase
when sensors and rollback exist.

## Four mechanisms

### 1. Sensors, not gates-only

Boundary assertions stay on (or sample) in production and **log** violations
with full context. Reject-without-telemetry wastes reality’s signal.
Use house observability; do not invent a monitoring stack unasked.

### 2. Bounded blast radius

Ship behind structural damage limits when the house has them: flags, canary,
reversible migrations (deprecate+grace, not silent DROP), rollback path
**tested before** the feature. Remote publish remains ASK ONCE.

Agent question: not “am I sure?” (unanswerable) but “who notices, how fast,
what does undo cost?”

### 3. Reality writes the tests

Each production invariant violation → immutable regression with literal
data + repair task from the log. Corpus grows by survivors, not only by
author imagination. Wire when house has incident→test path; else land the
case manually from the log.

### 4. Report TTD/TTR, not omniscient Done

| Metric | Meaning |
|--------|---------|
| **TTD** | Time-to-detect if this is wrong |
| **TTR** | Time-to-revert to prior good state |

Prefer a change with TTD 5m / TTR 1m over “perfectly verified” with TTD weeks.
If unknown: say unknown — do not invent numbers. Pre-deploy Done still means
local+relational evidence; reality-loop **adds** future-facing metrics for
ship/ops scope.

## Protocol (mandatory wrap)

```
REALITY-LOOP
1. SENSORS — boundary violations log with context (house tools).
2. BOUNDED BLAST — staged exposure + tested rollback when shipping.
3. REALITY→TESTS — prod violations become immutable regressions.
4. TTD/TTR — state detect/revert times (or unknown) on delivery.
```

## Honesty

No pre-contact protocol substitutes for contact. Further “total
unconditional” protocol layers after this are usually theater — progress
lives in deploy, measure, let traffic write the next case.
Do not invent CI/canary/mutation theater when absent; ASK or report gap.

## Related

- Chain: [`VERIFICATION-CHAIN.md`](VERIFICATION-CHAIN.md)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
