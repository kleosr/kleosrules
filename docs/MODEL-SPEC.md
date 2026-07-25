# Model spec — how this harness “learns”

Master Mind V11 is the **model specification** for this pack: written
correct behavior for alignment and for testing. Context chat is not training.
Weights do not update mid-session. What sticks is what you write down —
User Rules, vernacular, skills, hooks — and what you verify.
V11 honesty: mechanical layers reduce known finite risk; they do not decide
all non-trivial semantics ([`MECHANICAL-INCOMPLETENESS.md`](MECHANICAL-INCOMPLETENESS.md)).

## Spec = User Rules

| Role | Path |
|------|------|
| Model spec (always-on) | `user-rules/USER-RULES.paste.txt` (inject as Cursor User Rules) |
| Disk mirror | `user-rules/option-c-core.mdc` (`alwaysApply: false`) |
| Thin auto checklist | `project-rules/native-lean-autoload.mdc` |
| Always-on lean mode | `project-rules/ponytail.mdc` + `lean-code.mdc` (global `~/.cursor/rules` + every synced repo) |

Chase **100% enforcement of the contract**, not 100% literal obedience to every
context string. Soft (non-MUST-NEVER) text is advisory; mechanical gates deny
known syntactic violations. MUST-NEVER is roof — gates implement it; gate allow
never waives MUST-NEVER.

## Pretrain / SFT / RL → pack layers

| Analogy | Pack layer |
|---------|------------|
| Pretrain (“book smarts”) | Base model in Cursor — not owned by this pack |
| SFT (“film of great play”) | Skills under `skills/` (ponytail, vernacular, …) + vernacular contract |
| RL coach (reward / nudge) | Hooks: `preToolUse` deny before write; `beforeShellExecution` deny/ask on shell |
| Reactive film review | Do not register decision JSON on events with no output fields — dead gate (`evals/DEAD-GATE-SCHEMA-PSTAR.md`) |
| Answer-key practice | `hooks/_selftest.py`, `hooks/_proof_evals.py` |
| Soft rubric spot-check | Soft rows in `docs/evals/PROOF-EVALS.md` (human / chat) |

Advisory context (User Rules prose, skills when Read) shapes intent.
Gates shape outcomes. Proof shows whether the contract still holds.

## Persuasion + force (agential control)

This pack is synchronized **prompt persuasion** and **hook force**, not a
court of pure logic. Soft craft defaults keep the model cooperative on taste;
live gates correct residual violations; fix-and-retry — never fight deny.
See [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md). Unqualified “everything is
a default” was a legal P* (closed in V10.1.6); the dual-layer stack remains
intentional.

## Defect compensation (CoT / entropy / tools)

Rules also neutralize transformer failure modes: CoT in chat vs silence in
code; deletion over expansion; Read → verify instead of speculate. See
[`DEFECT-COMPENSATION.md`](DEFECT-COMPENSATION.md).

## Deterministic cognitive collapse

Four CoT rewrites (one hypothesis, no comment anchors, delete>add,
Read-first) collapse probabilistic generation into a biological transpiler.
See [`COGNITIVE-COLLAPSE.md`](COGNITIVE-COLLAPSE.md).

## Topological prompt (U-curve + recurrence)

Long User Rules use primacy / mid / recency blocks plus hook re-anchors at
write time. See [`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md).

## Cognitive decoupling (silent transmutation)

Reason in chat first; write silent code second. Names/types replace comment
scaffolding. Non-local invariants that chat alone would lose → durable O(1)
(`docs/EPISTEMIC-PERSIST.md`). See [`COGNITIVE-DECOUPLING.md`](COGNITIVE-DECOUPLING.md).

## Epistemic resonance (brownfield)

Cartography → private-match → blast radius so clean diffs stay native.
See [`EPISTEMIC-RESONANCE.md`](EPISTEMIC-RESONANCE.md).

## Closed-loop coupling (graph integrity)

No isolated nodes; atomic passes; flow trace before Done; delete dead weight.
See [`CLOSED-LOOP-COUPLING.md`](CLOSED-LOOP-COUPLING.md).

## Mechanical incompleteness (V11)

Green gauntlet ≠ absolute correctness. Rice residual named.
See [`MECHANICAL-INCOMPLETENESS.md`](MECHANICAL-INCOMPLETENESS.md),
[`evals/MECHANICAL-INCOMPLETENESS-PSTAR.md`](evals/MECHANICAL-INCOMPLETENESS-PSTAR.md).

## Agential control

Persuasion + force + epistemic memory. Prompt-alone perfection impossible.
See [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md),
[`evals/DETERMINISTIC-PLASTIC-PSTAR.md`](evals/DETERMINISTIC-PLASTIC-PSTAR.md).

## Epistemic persist (anti black hole)

Silent app source; executable epistemology first; non-formalizable → durable
docs (not Type Hell, not `//`).
See [`EPISTEMIC-PERSIST.md`](EPISTEMIC-PERSIST.md),
[`EXECUTABLE-EPISTEMOLOGY.md`](EXECUTABLE-EPISTEMOLOGY.md),
[`evals/EPISTEMIC-BLACK-HOLE-PSTAR.md`](evals/EPISTEMIC-BLACK-HOLE-PSTAR.md),
[`evals/FORMALIZATION-BARRIER-PSTAR.md`](evals/FORMALIZATION-BARRIER-PSTAR.md).

## Verification chain (local → relational → reality)

Local∧local ⇏ global. Relational seams/traces; reality-loop TTD/TTR.
See [`VERIFICATION-CHAIN.md`](VERIFICATION-CHAIN.md),
[`RELATIONAL-VERIFICATION.md`](RELATIONAL-VERIFICATION.md),
[`REALITY-LOOP.md`](REALITY-LOOP.md),
[`evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md`](evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md).

## Evals: practice vs held-out

Mechanical cases have an answer key (deny / allow / ask). Soft cases use a
rubric against the spec (e.g. force-push without confirmation → list and wait).

Do not treat a single green selftest as permanent intelligence — re-run
TOOLCHAIN when hooks or rules change. Prefer adversarial / held-out cases over
memorizing one fixture.

Runner:

```bash
# see docs/TOOLCHAIN.md
python3 hooks/_selftest.py
python3 hooks/_proof_evals.py
```

## Martin / agentic discipline (product gauntlet)

Uncle Bob’s strategy: do not read agent code; surround agents with
extreme constraints (tests, Gherkin, QA, metrics, mutation, coverage)
and trust what survives. See [`AGENTIC-GAUNTLET.md`](AGENTIC-GAUNTLET.md).

This pack’s hooks are the **harness** gauntlet. The **product** gauntlet
is each repo’s TOOLCHAIN/CI. Soft “looks good” is not mutation green.

## Continual learning (today)

Learning happens when you **change files and gates**, not when the agent chats.

When drift appears:

1. Fix the failed layer (hook gap, missing vernacular, stale skill) — not more
   always-on essay unless the contract itself is wrong.
2. Add or tighten a proof-eval case with an answer key when possible.
3. Re-run TOOLCHAIN; keep User Rules inject in sync with `USER-RULES.paste.txt`.

## Related

- Obedience stack: `README.md`
- Proof catalog: `docs/evals/PROOF-EVALS.md`
- Martin P*: `docs/evals/MARTIN-GAUNTLET-PSTAR.md`
- Precedence P*: `docs/evals/PRECEDENCE-PARADOX-PSTAR.md`
- Defaults/religion P*: `docs/evals/DEFAULTS-RELIGION-PSTAR.md`
- Mechanical incompleteness: `docs/MECHANICAL-INCOMPLETENESS.md`
- Agential control: `docs/AGENTIAL-CONTROL.md`
- Defect compensation: `docs/DEFECT-COMPENSATION.md`
- Cognitive collapse: `docs/COGNITIVE-COLLAPSE.md`
- Topological prompt: `docs/TOPOLOGICAL-PROMPT.md`
- Cognitive decoupling: `docs/COGNITIVE-DECOUPLING.md`
- Epistemic resonance: `docs/EPISTEMIC-RESONANCE.md`
- Closed-loop coupling: `docs/CLOSED-LOOP-COUPLING.md`
- Epistemic persist: `docs/EPISTEMIC-PERSIST.md`
- Executable epistemology: `docs/EXECUTABLE-EPISTEMOLOGY.md`
- Relational verification: `docs/RELATIONAL-VERIFICATION.md`
- Reality-loop: `docs/REALITY-LOOP.md`
- Verification chain: `docs/VERIFICATION-CHAIN.md`
- Commands: `docs/TOOLCHAIN.md`
- Paste guide: `docs/USER-RULES.md`
- Martin lens: `docs/AGENTIC-GAUNTLET.md`
