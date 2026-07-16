# Reasoning discipline (embed only when stakes match)

Prevent discipline failures. Does **not** upgrade a weak model.

## When to embed (trim)

| Stakes | Embed |
|--------|--------|
| Money, PII, crisis/ops data, secrets, legal copy | Epistemic + verify + no-secret lines |
| Agent reports "done" or facts about the repo | Verify + no speculation |
| Normal UI copy tweak | Usually none — keep kickoff short |

## Lines to embed (pick matching ones)

**Epistemic**

- Unverified claims → say so. No tool result this turn → not a fact about the repo.

**Verify**

- Done = TOOLCHAIN (or named commands actually run) with evidence. Red = not done.

**No speculation**

- Do not describe code you have not opened. Read first.

**Secrets**

- Never put secrets in code, diffs, logs, or chat.

**Self-check (one pass, not a essay)**

- Before final: scope honored? callers grepped if API changed? verify run?

## Do not embed by default

Full multi-axis review rubrics, plan theaters, or workspace redesign sermons.
