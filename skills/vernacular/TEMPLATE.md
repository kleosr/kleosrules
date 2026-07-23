# Vernacular contract (copy into the repo)

Put this at `.cursor/rules/vernacular.mdc` (recommended, `alwaysApply: true`) or `VERNACULAR.md`.

Fill every `TBD` from **existing** code — do not invent a dialect.

```md
---
description: Private vernacular for this repo — naming, files, visibility.
alwaysApply: true
---

# Vernacular

## Topology
- Shape: TBD (single app | apps+lib | existing packages only — no new monorepo)
- New files allowed under: TBD
- Never create: TBD (e.g. `**/use-cases/**`, `**/domain/**` unless already present)

## File names
- Pattern: TBD (e.g. `camelCase.ts` | `kebab-case.ts` | `snake_case.py`)
- Test files: TBD
- One export style: TBD (default export | named only)

## Types / classes
- Prefer: TBD (functions | classes | both)
- Class names: TBD
- Interfaces/types: TBD (`IFoo` banned? `Foo` / `FooProps`?)
- Visibility: TBD (private fields? module-private?)

## Functions
- Names: TBD (verbPhrase | snake_case)
- Async suffix: TBD
- Error/result style: TBD (throw | Result | null)

## Imports
- Order: TBD
- Alias: TBD (`@/` | relative only)

## Forbidden (quality = not these)
- Prose comments (Option C NO COMMENTS)
- New workspace/monorepo tooling
- Foreign Clean Architecture folders
```
