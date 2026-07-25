# Executable epistemology — preferred O(1) memory

Phase-1 response to the silent-complexity black hole
([`evals/EPISTEMIC-BLACK-HOLE-PSTAR.md`](evals/EPISTEMIC-BLACK-HOLE-PSTAR.md)).
Preferred path under [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md).

## Idea

Do not put tacit knowledge in app-source prose. Prefer **executable
contracts** Session 2 can read without walking the whole AST:

| Mechanism | Example load |
|-----------|----------------|
| Types / typestate | `process(tx: LockedDbTransaction)` — compiler guards the lock |
| Runtime asserts | Temporal / caller preconditions that types cannot state cheaply |
| Spec / test suite | Named behaviors and edges — structured comment, not AST noise |

Session 2 opens `*.spec.ts` / `*_test.go` / boundary types first — O(1)
intent — then implementation.

## Order

1. Encode in types if cheap.
2. Else assert / invariant at the seam.
3. Else test named as the invariant.
4. If still non-formalizable without Type Hell → durable prose surface
   (not app comments) — see formalization barrier kill.

## Related

- Persist: [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md)
- Barrier P*: [`evals/FORMALIZATION-BARRIER-PSTAR.md`](evals/FORMALIZATION-BARRIER-PSTAR.md)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
