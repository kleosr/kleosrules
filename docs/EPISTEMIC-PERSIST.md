# Epistemic persist — durable O(1) without source comments

Kill companion to [`evals/EPISTEMIC-BLACK-HOLE-PSTAR.md`](evals/EPISTEMIC-BLACK-HOLE-PSTAR.md)
and [`evals/FORMALIZATION-BARRIER-PSTAR.md`](evals/FORMALIZATION-BARRIER-PSTAR.md).
Extends [`COGNITIVE-DECOUPLING.md`](COGNITIVE-DECOUPLING.md),
[`EXECUTABLE-EPISTEMOLOGY.md`](EXECUTABLE-EPISTEMOLOGY.md).

## Defect

Phase A in chat + Phase B silent code produces zero-entropy *today* and
maximum reconstruct cost *tomorrow* when non-local invariants lived only
in the ephemeral session.

## Rule

**Silent application source. Durable epistemic surfaces.**

| Surface | Role |
|---------|------|
| App `.ts` / `.py` / … | No prose comments (MUST-NEVER) |
| Types / required params / asserts | Prefer — machine-checked O(1) |
| Tests named as invariants | Prefer — executable O(1) |
| `INVARIANTS.md` (module), ADR/`docs/`, `DEBT.md`, `HANDOFF.md`, package AGENTS note | Non-formalizable or Type-Hell-expensive intent |

## Executable first

When the constraint is formalizable without cyclomatic / type explosion,
encode it ([`EXECUTABLE-EPISTEMOLOGY.md`](EXECUTABLE-EPISTEMOLOGY.md)).
Do not invent typestate theater for Q3 calendars or vendor politics.

## Formalization barrier

Some trade-offs (temporary patches, hardware politics, toolchain bugs)
are **not** efficiently executable. Forcing them into types/tests
destroys lean; omitting them destroys Session 2. Route to durable prose
**outside** the app AST — that is not a waiver of NO COMMENTS.

## Before Done (wired / non-trivial)

If Phase A named a fragile constraint that Session 2 cannot recover from
local names alone → land it on a durable surface in the same change set
(executable preferred; markdown when non-formalizable). Incomplete
otherwise.

Do not invent Clean Architecture docs for CRUD. Do persist the lock /
ordering / “must not call from X” / “do not ‘optimize’ this” edges you
just introduced.

## Related

- Executable epistemology: [`EXECUTABLE-EPISTEMOLOGY.md`](EXECUTABLE-EPISTEMOLOGY.md)
- Cognitive decoupling: [`COGNITIVE-DECOUPLING.md`](COGNITIVE-DECOUPLING.md)
- Closed-loop: [`CLOSED-LOOP-COUPLING.md`](CLOSED-LOOP-COUPLING.md)
- Session continuity: skill `session-handoff`
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
