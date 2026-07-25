# Cognitive decoupling — silent transmutation

Breakthrough (2026-07-24): comments are cognitive scaffolding for
Transformers. Banning them without a prior reasoning surface collapses
quality. Clean code and full intelligence are symbiotic only if **reason
and transcript are decoupled**.

Companions: [`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md),
[`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md),
[`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md).

## Defect: textual scaffolding trap

Models write `// check null` to prepare the next logical tokens. Strip
comments with no prior thought surface → cognitive load collapse: dense,
broken, or over-compressed code.

## Two-phase protocol

### Phase A — Latent blueprint (chat / CoT)

Unrestricted natural language in the session: architecture, edge cases,
data flow, one hypothesis when debugging. Updates hidden state so the
solution is already thought before the first code character.

### Phase B — Silent transmutation (artifact)

Compiler mode. No prose comments. Semantic saturation: load that would
have been a comment moves into names, types, and small cohesive functions.
`// check user auth` → `isUserAuthenticated` (or equivalent vernacular).

## Three structural directives

1. **Semantic reflection** — Names of arbitrary useful length are the
   permanent comment. If a comment seems required, the name is wrong or
   the unit does too much — split or rename; do not add prose.
2. **Density restrictor** — Prefer no code; stdlib/repo reuse before new
   lines; fewest logical jumps. Without comments to patch confusion, the
   model must keep cyclomatic noise low for later turns.
3. **Intelligence shield (primacy)** — Identity: intelligence = solving
   hard problems with minimal structural entropy. Calibrates latent space
   toward high-level solve; execution gates only format the output.

## Symbiosis

| Goal | Mechanism |
|------|-----------|
| Zero comment drift in files | Thought stays in Phase A |
| Higher architecture quality | Names/types/size replace narrative |
| Absolute efficiency | Occam + reuse; no prose patches |

Do not forbid thinking. Channel cognitive load out of the **application
file** into session blueprint + code shape — and, when non-local, into a
**durable** repo surface (not only chat).

## Epistemic black hole (Session 2)

Chat is ephemeral. Syntax is O(N) for non-local invariants. Forbidding
prose in source without persisting fragile constraints elsewhere
guarantees future Lost-in-the-Middle. Kill: silent code ≠ silent repo —
[`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md),
[`evals/EPISTEMIC-BLACK-HOLE-PSTAR.md`](evals/EPISTEMIC-BLACK-HOLE-PSTAR.md).

## Related

- Cognitive collapse: [`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md)
- Defect compensation: [`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md)
- Topological U-curve: [`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md)
- Epistemic persist: [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md)
- Spec: [`MODEL-SPEC.md`](MODEL-SPEC.md)
- Epistemic resonance: [`EPISTEMIC-RESONANCE.md`](EPISTEMIC-RESONANCE.md)
