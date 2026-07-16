---
name: ui-structure
description: >-
  Enforces ordered, minimal UI structure: layout, spacing, size, separators,
  and DOM wrappers without hardcoded values or decorative clutter. Use when
  building or reviewing screens, components, forms, lists, or layouts where
  hierarchy, rhythm, or simplicity matters.
---

# UI structure

Order and rhythm first. Every box, gap, size, and divider must earn its place.

## When to load siblings

- Values / themes / CSS variables → `design-tokens`
- Marketing look, brand, hero, anti-generic visuals → `frontend-design`
- This skill owns structure, hierarchy, and cognitive load

## Invariants

- No arbitrary layout numbers in components. Spacing, size, radius, gap,
  width/height constraints, and separator thickness come from the project's
  token or spacing scale only.
- One clear reading order: primary action and primary content are obvious
  without scanning the whole page.
- One job per region. Nested regions that only add wrappers, cards, or
  borders are debt — delete them.
- Prefer fewer surfaces. A new `div`, card, panel, or section needs a
  structural reason (group, scroll, alignment, accessibility landmark), not
  decoration.
- Separators are rare. Prefer spacing and typography hierarchy. Use a
  divider only when two adjacent blocks would otherwise merge wrongly.
- Density follows content: tighten related items; leave clear air between
  unrelated ones. Never pad to "fill" empty space.
- Interaction cost stays low: short paths, obvious labels, no competing CTAs
  in the same band.

## Structure pass (before styling)

1. Name the user's next useful action and the one piece of information that
   must be found first.
2. Sketch regions top-down: page → section → group → control. Stop when
   another wrapper would not change reading order or accessibility.
3. Assign each region one responsibility. Merge siblings that share the
   same job; split only when jobs conflict.
4. Map spacing to the existing scale (e.g. stack vs inset vs section gap).
   Do not invent one-off gaps.
5. Decide separators last: spacing first, rule/line only if grouping fails.
6. Check states that break order: empty, loading, error, long content,
   narrow viewport. Hierarchy must survive all of them.

## Harm checklist (reject or fix)

- Wrapper soup: wrappers with no layout, a11y, or scroll role
- Nested cards / bordered boxes inside bordered boxes
- Hardcoded `px`/`rem`/`%` sizes that express design (use tokens)
- Decorative dividers between every row or section
- Multiple peer headings or CTAs fighting for attention
- Inconsistent alignment or gap rhythm across sibling items
- Controls that look alike but do different jobs
- Hidden primary actions behind menus when the screen has room
- "Minimal" that removes labels, feedback, or affordances people need

## Make / change / remove

- **Make**: introduce the smallest structure that preserves order; wire
  sizes and gaps through tokens in the same change.
- **Change**: update the owning layout primitive or token, then every
  consumer in scope — no local one-off overrides.
- **Remove**: delete unused wrappers, orphan separators, and dead size
  utilities end to end; leave no empty style hooks.

## Verification

- Trace layout values with `design-tokens` discipline in the affected scope.
- Confirm a new user can answer: Where am I? What matters? What do I do next?
- Re-check empty/loading/error and a narrow viewport.
- Report regions kept, wrappers/separators removed, token gaps closed, and
  any remaining hardcoded values with reasons. Unverified surfaces stay
  unverified.
V