---
name: design-tokens
description: >-
  Traces and completes design-token and UI-data changes from source of truth
  through every consumer. Use when creating, changing, auditing, or removing
  CSS variables, themes, component styles, visual values, or hardcoded UI data.
---

# Design tokens

One source of truth, fully wired or fully removed.

## Invariants

- Raw design literals live only in the canonical primitive-token registry.
  Consumers use semantic tokens that describe intent, not a palette value.
- Preserve the repository's token system and naming. Never add a parallel
  theme, token registry, styling layer, or generic configuration store.
- A literal cannot disappear entirely: centralize it at its owning source.
  CSS mechanics such as `auto`, `inherit`, `currentColor`, `none`, `0`, `100%`,
  and `1fr` are not design decisions; arbitrary visual values are.
- Tokenize color, typography, spacing, sizing, radius, border, shadow, motion,
  opacity, breakpoint, layer, and asset choices when they express design.
- Product data belongs to its established owner: props, domain/API data,
  configuration, i18n catalog, CMS, feature flag, or asset manifest. Do not
  move a literal into a new abstraction merely to hide it.

## Trace before editing

1. Find the canonical primitive and semantic token definitions, theme or mode
   overrides, build/config exports, and component consumption path.
2. Search the entire affected scope for the token, aliases, raw value, fallback
   values, generated names, framework utilities, and typed exports.
3. Map the observed flow:

```text
primitive → semantic token → theme/mode → build or typed export
          → component/utility → state/variant → rendered UI
```

4. Identify all states and surfaces: default, hover, focus, active, disabled,
   loading, error, responsive layouts, dark/high-contrast modes, and reduced
   motion where relevant.
5. Unknown links are gaps. Do not claim or edit consumers that were not found.

## Make one complete change

### Create

- Add the primitive only when no equivalent exists.
- Add a semantic token named for its role; components must not consume palette
  primitives directly.
- Wire every required theme, export, framework mapping, component state, and
  responsive surface in the same change.
- Replace in-scope raw literals and fallbacks with the canonical token.

### Change

- Change the owning definition, then verify aliases and every consumer.
- Preserve public names unless a rename was requested; migrate all consumers
  atomically when renaming.
- Do not patch one component with a local override to mask an incomplete token.

### Remove

- Search definitions, aliases, exports, generated files, utilities, consumers,
  stories, tests, and documentation.
- Remove the entire path from source to rendered use. Zero consumers means
  delete the token; do not retain compatibility aliases without a real caller.
- Regenerate derived artifacts with the repository's existing command.

## Hardcoded UI data

Trace non-style UI values to their owner with the same discipline:

```text
owner → contract/type → transport or adapter → component → rendered state
```

No placeholder copy, fake counts, prices, URLs, IDs, options, or feature state
in production paths. If no owner exists, establish the smallest explicit owner
required by the product and wire it end to end.

## Verification

- Run the repository's prescribed lint, type, test, build, and token-generation
  commands; do not invent a suite.
- Search again for stale names, orphan definitions, forbidden raw literals,
  fallback values, and hardcoded UI data in the affected scope.
- Verify representative rendered states and responsive/theme variants using the
  project's existing visual tooling when available.
- Report the source of truth, traced consumers, completed or removed path,
  exceptions with reasons, and commands actually run. Unverified surfaces stay
  explicitly unverified.
