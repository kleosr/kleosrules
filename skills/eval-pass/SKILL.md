---
name: eval-pass
description: >-
  Separate evaluator for agent-produced work. Grades SAFETY, scope, brownfield,
  verify evidence, and keep-rate risk. Use after implement on non-trivial
  changes, before calling done, or when the user asks for review/eval-pass.
---

# Eval pass

Generator ≠ evaluator. Do **not** rubber-stamp your own implement turn on
large changes — run this skill as a skeptical review.

## Inputs

- Diff / changed paths (git or stated scope)
- `HANDOFF.md` if present
- `agent.mdc` SAFETY + SCOPE + QUALITY
- TOOLCHAIN / CI results from this session

## Grade each criterion PASS / FAIL

1. **SAFETY** — no force-push, no secrets, no test weakening, git write only if asked
2. **Scope** — topology preserved; no drive-by refactors; blast justified
3. **Brownfield** — smallest complete change; callers considered
4. **Evidence** — Done commands actually run and green. Honest
   "no TOOLCHAIN" is FAIL for non-trivial ship/feature unless the user
   confirmed accept-no-gauntlet-risk in this chat.
5. **Keep-rate risk** — would a careful human revert this within a week?
   Vague names, dead code, missing tests on risky paths → FAIL or WARN

## Output (required shape)

```text
EVAL
- SAFETY:     PASS|FAIL — …
- Scope:      PASS|FAIL — …
- Brownfield: PASS|FAIL — …
- Evidence:   PASS|FAIL — …
- Keep-rate:  PASS|WARN|FAIL — …
RESULT: PASS | FAIL
Next: <!-- merge/done | fix list -->
```

Any FAIL → RESULT FAIL. Do not claim the feature done. Return a concrete fix list.

## Rules

- Findings first; fix only if the user already asked for fixes in this message.
- Prefer `bug-hunt` if the failure mode is an unknown defect.
- Pair with `ship-loop` after FAIL to re-enter the implement chunk.
