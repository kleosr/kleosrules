---
name: vernacular
description: >
  Load and enforce the project's private code vernacular — file names, class /
  type / function naming, folders, import style, visibility. Use before writing
  or editing application code when the repo has VERNACULAR.md or
  .cursor/rules/vernacular.mdc, or when the user asks for custom syntax /
  naming / private-looking code / highest-quality native match.
---

# Vernacular

Native Lean quality = **this repo’s private dialect**, not a global Clean Code essay.

## Before writing application code

1. Look for, in order, and **Read** the first that exists:
   - `.cursor/rules/vernacular.mdc`
   - `VERNACULAR.md`
   - `docs/VERNACULAR.md`
2. If found: follow it for file names, folders, classes/types, functions, imports,
   visibility (private/internal), error patterns. It outranks generic taste.
3. If missing: private-match **sibling files only** (Option C NATIVE LEAN). Do not
   invent a new dialect. Optionally offer to scaffold from
   `~/.cursor/skills/vernacular/TEMPLATE.md` if the user wants a contract.
4. Still obey Option C: prefer no code, no prose comments, no monorepo theater.

## What vernacular controls

- File and folder names (e.g. `foo_bar.ts` vs `FooBar.ts` vs `foo-bar.ts`)
- Class / type / interface / enum naming and when each is allowed
- Function / method naming; getters; factory patterns the repo already uses
- Visibility: `private` / `internal` / module-private defaults
- Import order and path style (`@/`, relative, package name)
- Where new files may be created (and where they must not)

## Anti-patterns

- Applying another repo’s vernacular here
- Introducing PascalCase services / UseCase folders when the repo is flat functions
- “Improving” names into a foreign style on a surgical fix

Off only: user says stop vernacular / ignore vernacular for this task.
