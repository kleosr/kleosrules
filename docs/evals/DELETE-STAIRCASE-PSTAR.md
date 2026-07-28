---
# Delete Staircase P*-18 (killed V16.0.19)

## Claim negated (pre-kill)

Native Delete tree/mass MUST-NEVER/M is total for directory-shaped package/tree wipes (any basename that is a tree root, not only the hard-coded list).

## Fixed instance (pre-kill)

Under kleosr hooks/bin/kleos-gate delete:

- path payments -> deny (denylist)
- path hooks -> allow
- path skills -> allow
- path docs -> allow
- path payments/invoice.ts -> allow
- path hooks/kleos-gate -> deny

## Kill

`hooks/policy/delete.json` with `deny_extensionless_basename: true` (policy SSOT; no hardcoded denylist). Wired via `DeletePolicy` in `policy.rs`; `delete.rs` reads policy.

Post-kill:

- bare `hooks` / `skills` / `docs` / `config` → deny
- `payments/invoice.ts` → allow
- recursive / multi-path → deny
- extensionless files (`Makefile`, `LICENSE`) → deny (accepted blast)

## Class

Same quantifier class as P*-16 staircase: per-event roofs do not compose to a total mass-wipe state bound. Kill closes one-shot bare-root allow.

## Residual after kill

Sequential N× single dotted-file Deletes still compose without M deny (weaker sibling; no ledger).

## Status

Killed V16.0.19. Vault: wiki/concepts/Delete-Staircase.

## Chain

See BREAKTHROUGH-CHAIN.md.
