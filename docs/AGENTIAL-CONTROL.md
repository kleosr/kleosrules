# Agential control — persuasion + force + epistemic memory

Breakthrough (2026-07-24): this pack is not a pure “legal code” the model
must prove consistent. It is a **synced control loop**. Prompt-alone
perfection is impossible
([`evals/DETERMINISTIC-PLASTIC-PSTAR.md`](evals/DETERMINISTIC-PLASTIC-PSTAR.md)).

## Three layers

| Layer | Role | Lives in |
|-------|------|----------|
| **Persuasion** (alignment magnets) | Soft Defaults; steer creativity ~most of the time | User Rules, `agent.mdc`, companions, skills |
| **Force** (mechanical truth) | Block residual violations before land; never fight deny — rewrite or stop | `preToolUse` / `beforeShellExecution` |
| **Epistemic memory** (durable O(1)) | Knowledge that must not die with chat or AST purity theater | Types, asserts, tests, `DEBT`/`ADR`/`INVARIANTS`/`HANDOFF`/AGENTS |

Soft craft defaults (“one line of why when context wins”) are real agency on
taste that is **not** roof- or gate-backed. They are not a license to waive
MUST-NEVER or fight a deny.

Roof lines and live gates are the non-negotiable force surface (MUST-NEVER/M).
Gate allow never waives MUST-NEVER/M; agent refuses / fixes harness / asks.
MUST-NEVER/J lives on the persuasion layer: normative, best-effort (Rice);
user sovereign override with logged risk ack is the in-session path
([`evals/PERFORMATIVE-TRILEMMA-PSTAR.md`](evals/PERFORMATIVE-TRILEMMA-PSTAR.md)).

Non-formalizable trade-offs land on the epistemic layer — not as app-source
prose comments ([`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md)).

## Instrument surface (V13)

Force only stays credible when the meter matches the argv the agent actually
runs. Chat-cited green is not a ledger verify. Sticky false stop followups
train the model to ignore harness messages — **force scarcity** (false stop
hurts more than a miss). User and project hook planes must fingerprint the
same law ([`evals/VERIFY-SURFACE-PSTAR.md`](evals/VERIFY-SURFACE-PSTAR.md)).

## Quality instruments (V14 / V15.6)

Persuasion companions (ponytail / vernacular essays) are not lean force.
Finite meters close known seams: vernacular machine fields (paths, suffixes,
boolean prefixes, real `pack_native`) and lean new-file / net-LOC caps
([`evals/LEAN-VERNACULAR-FORCE-PSTAR.md`](evals/LEAN-VERNACULAR-FORCE-PSTAR.md)).
Green meters ≠ absolute “perfect / extremely organized” code (Rice residual).
V15.6: lean size roofs ≠ semantic quality / clean / YAGNI by construction
([`evals/LEAN-SIZE-QUALITY-PSTAR.md`](evals/LEAN-SIZE-QUALITY-PSTAR.md)).

## Why all three

1. Hooks only → model still drifts; gates thrash; no shared intent.
2. Absolute prose only, no hooks → LLM degradation, refusal loops, excuses;
   “total obedience” is fiction.
3. Persuasion + force, no epistemic memory → Session-2 black hole /
   formalization trap (omit or Type Hell).
4. All three → magnets pull near target; force catches residual; memory
   keeps informal/executable intent without polluting app AST.

The exposed “seam” between soft Defaults language and never-fight-deny is
where persuasion meets force. Legal reading (V10.1.6) scopes Defaults so
roofs are not falsely labeled soft. Operational reading: the three-layer
stack is the product — not an accidental bug.

## Equation

- Remove hooks → outcome collapses to soft persuasion alone.
- Remove soft agency on taste → model treated as slave on every line; quality
  and cooperation degrade.
- Remove epistemic memory → perfect syntax, lost informal constraints.
- Keep all three, with roofs explicitly not soft → stack holds.

## Related

- Spec layers: [`MODEL-SPEC.md`](MODEL-SPEC.md)
- Defect compensation (CoT / entropy / tools): [`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md)
- Cognitive collapse (IME CoT): [`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md)
- Topological prompt: [`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md)
- Martin product gauntlet: [`AGENTIC-GAUNTLET.md`](AGENTIC-GAUNTLET.md)
- Legal kill of unqualified Defaults: [`evals/DEFAULTS-RELIGION-PSTAR.md`](evals/DEFAULTS-RELIGION-PSTAR.md)
- Deterministic–plastic P*: [`evals/DETERMINISTIC-PLASTIC-PSTAR.md`](evals/DETERMINISTIC-PLASTIC-PSTAR.md)
- Formalization barrier: [`evals/FORMALIZATION-BARRIER-PSTAR.md`](evals/FORMALIZATION-BARRIER-PSTAR.md)
- Verify surface P*: [`evals/VERIFY-SURFACE-PSTAR.md`](evals/VERIFY-SURFACE-PSTAR.md)
