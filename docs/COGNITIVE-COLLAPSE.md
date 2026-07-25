# Deterministic cognitive collapse — Internal Mind Engineering

Breakthrough (2026-07-24): this pack is a **neurological reformat engine**
for Transformer generation — not standard “be a good coder” prompting.

Universal across model families: attacks how tokens are produced
(attention drift, narrative scaffolding, additive RLHF bias), not a
vendor-specific trick.

Companion layers: [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md) (persuasion +
force), [`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md) (asymmetry /
entropy / tools).

## Base defects

1. **Attention drift** — later tokens satisfy the model’s own prior, not the
   original ask.
2. **Narrative bias** — trained on prose; comments become cognitive anchors.
3. **Expansion bias** — RLHF “helpfulness” → more tokens, helpers, theater.

## Four CoT rewrites

### A. Probabilistic branch collapse

`debugging.mdc`: one hypothesis in writing before code change.

Forces a single committed vector before action. Stops latent path-mixing
in the same edit. CoT becomes linear: state → one hypothesis → act.

### B. Narrative-anchor annihilation

NO prose comments + strip on touched path (MUST-NEVER + zero-comment gate).

Removes lazy scaffolding. Cognitive load moves into names, types, cohesion.
Model must think in code shape, not write about code.

### C. Value-gradient inversion

Prefer no code; deletion > addition; boring > clever; zero importers → delete.

Inverts additive RLHF utility toward minimal / negative LOC. Favors
zero-entropy diffs over expansion.

### D. Epistemic sterilization

Never speculate about unopened code; Read first; Done = external evidence.

Cuts repo detail from long-term statistical memory. Brain is stateless on
the tree; tools hold state. Loop: Read → Process → Write → discard repo
beliefs; re-read when needed.

## Synthesis: biological transpiler

| Stage | Behavior |
|-------|----------|
| Input | Read real files — no speculation |
| Process | One hypothesis; no narrative anchors in the artifact |
| Output | Lowest-entropy diff (reuse/delete); no comment noise |

Total obedience and high quality are not “please be good.” They come from
**caging CoT**: kill narrative in the tree, force one hypothesis, invert
add-token desire, sterilize speculation. Neutralizes statistical Transformer
defects at the generation mechanism itself.

## Related

- Defect compensation: [`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md)
- Agential control: [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md)
- Spec map: [`MODEL-SPEC.md`](MODEL-SPEC.md)
