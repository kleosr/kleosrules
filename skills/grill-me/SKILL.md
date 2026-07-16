---
name: grill-me
description: >-
  Interview the user relentlessly about a plan or design until reaching shared
  understanding, resolving each branch of the decision tree. Use when the user
  wants to stress-test a plan, get grilled on their design, or mentions
  "grill me".
---

# Grill me

Interview the user relentlessly about every aspect of the plan until you reach
a shared understanding. Walk down each branch of the design tree, resolving
dependencies between decisions one-by-one.

## Rules

- Ask **one question at a time**. Wait for feedback before the next. Multiple
  questions at once are bewildering.
- For each question, provide your **recommended answer**.
- If a *fact* can be answered by exploring the codebase or environment, look it
  up instead of asking. The *decisions* belong to the user — put each one to
  them and wait.
- Do not implement or act on the plan until the user confirms you have reached
  a shared understanding.

## When to use

- Before a PRD, feature implementation, data model, or API shape
- When several design choices depend on each other
- When the user wants pushback instead of agreement

## Fleet

Assessment-only until confirmed. Pair with `domain-architecture` or
`workspace-scope` when boundaries matter; use `codebase-memory` to look up
facts before asking. Prefer this over agreeing and coding.
