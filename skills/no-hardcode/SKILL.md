---
name: no-hardcode
description: >-
  Prevents technical debt from hardcoded values across frontend, backend,
  config, and infrastructure. Use when introducing or reviewing literals,
  magic numbers, URLs, credentials, feature flags, copy, timeouts, IDs,
  environment values, or duplicated constants anywhere in the stack.
---

# No hardcode

Every meaningful literal has one owner. Fully wired or fully removed.

## Intent

Hardcoding is debt when a value can change, differs by environment, is shared,
expresses policy, or hides intent. Correctness means the value lives at its
true owner and every consumer reads it from there — not a second copy.

## Owner map (pick the real one)

| Kind of value | Canonical owner |
|---------------|-----------------|
| Secrets, API keys, passwords | Secret store / env — never committed |
| Host, ports, service URLs, feature toggles | Environment config / flags service |
| Business rules, limits, statuses, domain IDs | Domain module / policy constants |
| API/event contract enums and schemas | Shared contract package or schema |
| User-facing copy | i18n / CMS / content owner |
| Visual size, color, spacing, motion | Token system → use `design-tokens` |
| Layout rhythm and wrapper structure | Structure rules → use `ui-structure` |
| Build/tooling knobs | Toolchain / project config files |
| Persistence names (tables, topics) | Migration / infra definition as SSOT |

Never invent a parallel `constants.ts`, god-config, or “utils dump” to hide
literals. Extend the existing owner; do not create a second constitution.

## Allowed inline (not debt)

These may stay local when they carry no product/policy meaning:

- Language/framework mechanics (`0`, `1`, `null`, empty init, loop indexes)
- True mathematical identities and unit conversions with stable meaning
- Test fixtures that intentionally encode a scenario (keep them local to the test)
- Values that *are* the owning definition itself (the single declaration site)

If you must justify an inline value, one line of why is enough. Silence implies
it belongs at an owner.

## Forbidden patterns

- Credentials, tokens, connection strings, or private URLs in source
- Environment-specific hosts/paths baked into application code
- Magic numbers for timeouts, retries, limits, prices, quotas, thresholds
- Duplicated stringly status/type values across FE and BE
- Placeholder product data in production paths (fake users, counts, prices)
- Copy-pasted “temporary” literals left after a spike
- Fallbacks that reintroduce a hardcoded value “just in case”
- Abstractions whose only job is to wrap one literal with no second consumer

## Trace before editing

```text
literal → meaning → owner → export/config load → consumers → runtime effect
```

1. Name what the value means and who should own it if it changes.
2. Grep the affected scope for the raw value, aliases, and near-duplicates.
3. Prefer the repository’s existing pattern (env schema, settings module,
   domain const, i18n key, token, flag). Match it; do not redesign ownership.
4. Unknown owner → gap. Establish the smallest real owner required by the
   product, then wire consumers. Do not guess.

## Make / change / remove

### Create

- Declare once at the owner with a name that states intent.
- Validate at the trust boundary (env parse, schema, config boot).
- Replace every in-scope raw occurrence in the same change.
- Cross-boundary values: update contract + producer + consumer together;
  use `system-wiring` when hops span UI/API/workers.

### Change

- Change the owning definition first.
- Migrate all readers atomically; no mixed old/new literals.
- Preserve public names unless rename was requested.

### Remove

- Delete the owner only when zero consumers remain.
- Remove dead env keys, unused flags, orphan i18n keys, and stale docs in
  the same sweep when they are part of the path.
- No compatibility alias without a real caller.

## Debt prevention checks

- Would a second environment need a code edit? → config/env owner.
- Would product/legal/design change this without a deploy preference? →
  content, flag, or token owner — not a buried literal.
- Do FE and BE both need it? → shared contract or shared package, not twins.
- Is this a secret? → secret store; rotate if it ever touched git/chat/logs.
- Is this visual? → `design-tokens`, not ad-hoc CSS/JS numbers.

## Verification

- Search again for the raw value and obvious synonyms in the touched scope.
- Run the repo’s prescribed lint/type/test/config validation; do not invent
  a suite.
- Boot or exercise the path that loads config when the change depends on it.
- Report: owner chosen, consumers updated, leftovers with reasons, commands
  actually run. Unverified surfaces stay explicitly unverified.
