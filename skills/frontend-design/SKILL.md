---
name: frontend-design
description: >-
  Quality-first frontend and landing-page design constraints: composition,
  brand, typography, hero, cards, motion, anti-generic AI looks. Use when
  building or redesigning UI, landing pages, marketing surfaces, or visual
  React/CSS work. Skip when matching an existing design system.
---

# Frontend design

Avoid generic, overbuilt layouts. Prefer one clear visual direction.

## Hard rules

- **One composition**: First viewport reads as one composition, not a
  dashboard (unless it is a dashboard).
- **Brand first**: On branded pages, brand/product name is hero-level —
  not only nav text. No headline should overpower the brand.
- **Brand test**: If the first viewport could belong to another brand
  after removing the nav, branding is too weak.
- **Typography**: Expressive, purposeful fonts. Avoid default stacks
  (Inter, Roboto, Arial, system).
- **Background**: Not flat single-color only — gradients, images, or
  subtle patterns for atmosphere.
- **Full-bleed hero**: Landing/promo heroes are edge-to-edge by default.
  No inset heroes, side-panel heroes, rounded media cards, tiled
  collages, or floating image blocks unless the design system requires it.
- **Hero budget**: First viewport usually = brand, one headline, one
  short supporting sentence, one CTA group, one dominant image. No stats,
  schedules, address blocks, promos, or secondary marketing in viewport 1.
- **No hero overlays**: No floating badges, promo stickers, chips, or
  callout boxes on hero media.
- **Cards**: Default no cards. Never in the hero. Cards only as
  containers for user interaction. If border/shadow/bg/radius can go
  without hurting interaction, remove them.
- **One job per section**: One purpose, one headline, usually one short
  supporting sentence.
- **Real visual anchor**: Show product, place, atmosphere, or context.
  Decorative gradients alone are not the main idea.
- **Reduce clutter**: No pill clusters, stat strips, icon rows, boxed
  promos, or competing text blocks.
- **Motion**: At least 2–3 intentional motions for visually led work;
  presence and hierarchy, not noise.
- **Color**: Clear direction via CSS variables. Avoid AI-default looks:
  purple-on-white / purple-indigo gradients; warm cream (#F4F1EA) +
  serif + terracotta; broadsheet hairline / zero-radius newspaper
  columns. Avoid bias to dark mode, purple, glow, rounded-full pills,
  multi-layer shadows, emojis.
- **Responsive**: Desktop and mobile both load and read correctly.

## React

Prefer modern patterns the team already uses (`useEffectEvent`,
`startTransition`, `useDeferredValue` when appropriate). Do not add
`useMemo`/`useCallback` by default; follow the repo's React Compiler
guidance.

## Exception

Inside an existing website or design system: preserve established
patterns, structure, and visual language.
