---
name: lean-code
description: >-
  Keeps files at the smallest complete LOC without breaking behavior: delete
  dead code, reject over-engineering, and ban narrative comments. Use when
  writing, refactoring, reviewing, or cleaning code for bloat, wrappers,
  premature abstractions, or comment noise.
---

# Lean code

Smallest complete change. Names carry intent. No theater.

## Goal

Minimize lines of code in the touched files without changing externally
observable behavior, unless the task explicitly changes behavior. Lean means
fewer moving parts that still pass the repo's real checks — not code golf,
not clever compression that hurts reading or correctness.

## Invariants

- Prefer deletion and inlining over new helpers, layers, or “clean”
  indirection.
- No abstraction before the third real repetition in this codebase.
- No features, refactors, or files beyond the task.
- Zero importers → delete; do not invent callers.
- Names replace comments. If a block needs a paragraph to explain it, rename
  or reshape the code until it does not.
- No narrative comments: no section banners, no “what this does”, no commented-
  out code, no TODO/FIXME left as chat in source.
- Comment exceptions only when SAFETY or toolchain force them: `// SAFETY:`,
  required suppressions with a real reason, workaround + link, or public-API
  docs the project already requires. Nothing else.

## Over-engineering is debt (reject)

- Generic base classes, wrappers, or “flexible” options for one caller
- Indirection that only forwards arguments
- Config/plugin/factory machinery without a second proven need
- Duplicate types/adapters that mirror an existing module
- Premature performance caches or memoization the team does not already use
- Extra files whose only job is re-export or thin pass-through
- Patterns copied from blogs that the repo does not already follow

## Shrink pass

1. State the required behavior and the smallest surface that owns it.
2. Delete dead code, unused exports, unreachable branches, and stale comments
   in scope.
3. Inline one-shot helpers used once; keep a function only when reuse or a
   name clearly reduces complexity.
4. Collapse needless wrappers and pass-through modules.
5. Flatten control flow: early returns beat deep nesting when shorter and
   clearer.
6. Keep one representation of each idea; remove parallel “just in case” paths.
7. Stop when further deletion would change behavior, obscure a trust boundary,
   or fight TOOLCHAIN/house style.

## Make / change / remove

- **Make**: write the direct path first. Extract only after real repetition.
- **Change**: edit the owning code; do not add a parallel “cleaner” layer
  beside it.
- **Remove**: delete the whole unused path (code, types, tests that only
  existed for it, re-exports). Partial leftovers are debt.

## Verification

- Behavior: run the repo's prescribed checks for the touched scope; do not
  invent a suite.
- Diff: prefer net LOC down or flat for equal behavior. Growth needs a clear
  behavior reason.
- Grep the touched files for comment markers and commented-out blocks; remove
  what is not an allowed exception.
- Report: what was deleted or inlined, what stayed and why, commands run.
  Unverified behavior stays unverified.
