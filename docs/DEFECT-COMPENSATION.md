# Defect compensation — CoT, entropy, stateful tools

Breakthrough (2026-07-24): this pack does not “teach the LLM to program.”
It **neutralizes transformer failure modes** with asymmetric CoT, entropy
caps, and tool-backed state.

Companion to [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md) (persuasion + force).

## 1. Cognitive asymmetry (inner CoT vs artifact)

**Defect:** Models use prose comments as cognitive scaffolding; comments
rot, noise the repo, and hide weak names.

**Compensation:** NO prose comments in application code (MUST-NEVER +
zero-comment gate) + think in chat (hypotheses, evidence, plans). Debugging:
one hypothesis in writing before edits (`debugging.mdc`).

| Surface | Allowed |
|---------|---------|
| Chat / session CoT | Hypotheses, plans, citations, tradeoffs |
| Repo artifact | Silence — names and structure carry intent |

Quality shifts from narrative comments to semantic names and smallest diffs.
CoT stays in the session; the tree stays pure.

## 2. Entropy restrictor (high quality = what does not exist)

**Defect:** Autoregressive bias expands — helpers, speculative errors,
Clean Architecture theater, LOC growth.

**Compensation:** Native Lean / ponytail floors:

- Prefer no code; deletion > addition; zero importers → delete
- No abstraction before the third real repetition
- Shortest correct private-native diff; grep callers before shared change

High quality here means **what the agent prevents from existing** (persuasion
target), not a proven gate outcome. Entropy of the tree should fall or stay
flat per task. Lean meter only caps size — green lean ≠ clean / YAGNI
([`evals/LEAN-SIZE-QUALITY-PSTAR.md`](evals/LEAN-SIZE-QUALITY-PSTAR.md)).

## 3. Apparent oracle (stateful tools, stateless brain)

**Defect:** Long-session hallucination; speculation about unread code.

**Compensation:**

- Never speculate about code not opened — Read first
- Done = house gauntlet evidence; soft “looks good” ≠ Martin confidence
- No gauntlet + land code → ASK ONCE (accept-risk or wire verify)

Forced loop: Read → Hypothesis → Tool → Verify. Creativity that invents
state is shut off; the filesystem and TOOLCHAIN/CI hold truth.

## Synthesis

| Transformer defect | Pack compensation |
|--------------------|-------------------|
| Expansion / over-engineering | YAGNI + entropy restrictor |
| Hallucination / stale attention | Read-first + external verify |
| Narrative scaffolding in code | No prose comments; CoT in chat |

Max quality is not freer creation. It is CoT caged in read → hypothesize →
minimal diff → mechanical verify.

Deeper mechanism (four CoT rewrites → biological transpiler):
[`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md).
Two-phase think→silent code: [`COGNITIVE-DECOUPLING.md`](COGNITIVE-DECOUPLING.md).

## Related

- Cognitive collapse: [`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md)
- Cognitive decoupling: [`COGNITIVE-DECOUPLING.md`](COGNITIVE-DECOUPLING.md)
- Agential control: [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md)
- Martin gauntlet: [`AGENTIC-GAUNTLET.md`](AGENTIC-GAUNTLET.md)
- Spec map: [`MODEL-SPEC.md`](MODEL-SPEC.md)
