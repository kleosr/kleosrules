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

## Force vs persuasion

Hooks enforce **machine fields** only (when a contract exists), via
`deny-vernacular-drift` / `gate-write`:

- `file_name_pattern` (incl. real `pack_native`: snake/kebab + PascalCase
  `.tsx`/`.jsx`/`.vue`/`.svelte` components)
- `allowed_kinds` (with `domain.kind.ext`)
- `allowed_path_prefixes` (topology allow-list)
- `forbidden_class_suffixes` (theater class endings)
- `class_pattern` / `function_pattern` / `constant_pattern`
- `boolean_prefixes` (glued-prefix + boolean-shaped bare names)

Prose sections (import order, visibility defaults, “prefer functions” essays)
are **soft** unless expressed as machine fields. Do not claim the gate enforces
ungated prose.

## What vernacular controls

- File and folder names (gated when fields set)
- Class / type naming; forbidden theater suffixes when listed
- Function naming; boolean prefixes when listed
- Where new files may be created (`allowed_path_prefixes`)
- Soft: import order, visibility, cohesion — private-match siblings

## Anti-patterns

- Applying another repo’s vernacular here
- Introducing UseCase / Repository folder theater when the repo forbids them
  (React PascalCase components are OK under `pack_native` + `.tsx`)
- “Improving” names into a foreign style on a surgical fix
- Claiming always-on companions alone force dialect depth

Off only: user says stop vernacular / ignore vernacular for this task.
