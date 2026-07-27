# P* — Lean Size ≠ Semantic Quality (P*-15)

Finished unconditional counterexample. **Killed in V15.6** by claim scope.
No new Rust/policy meters (finite size roofs stay gameable on the quality axis).

## Verdict

Lean meter + no-comments + vernacular + always-on ponytail do **not** make
high-quality / clean / YAGNI code by construction. M surfaces measure size and
syntax roofs; semantic quality stays soft persuasion (J).

## Claim (C) — pre-fix

Native Lean + ponytail always-on + lean meter (`hooks/policy/lean.json`) +
no-comments gate + vernacular force make the agent write, by construction,
high-quality, low-LOC, clean (organized / real YAGNI) code.

## Claim (C′) — post-fix

Master Mind V15.6: lean meter + comment/vernacular roofs are **finite size and
syntax instruments** (proven M). Soft ladder (≤20-line taste, reuse, cohesion,
names with domain sense) is J persuasion — chase it; do not claim gates or
companions guarantee ∀ semantic quality. Green lean ≠ clean / YAGNI.

## Instance (pre-kill)

1. Caps active: `new_file_loc: 120`, `net_delta: 200`, no-comments, no vernacular
   contract → names ungated.
2. Ask: add email validation on signup form.
3. Diff: one new `.ts` at 119 LOC, net +119, zero prose comments, generic names
   (`handleSubmit`, `tmp`, `utilCheck`), duplicated regex/branches, nested block,
   no real test.
4. `kleos-gate` exit 0; artifact is mediocre and non-YAGNI — legal under C as
   “gates + companions = high quality.”

## Failure (by construction)

- Evidence: `hooks/policy/lean.json` (numeric LOC only);
  `docs/evals/LEAN-VERNACULAR-FORCE-PSTAR.md` residual;
  `skills/ponytail/SKILL.md` (companions do not alone deny bloat / extreme quality).
- What always happens: any new CODE_EXT ≤119 LOC / net ≤200 without comments
  passes the lean meter regardless of cohesion or YAGNI.
- Why ¬C: C asserts high quality / clean by construction; P* satisfies every M
  instrument and violates colloquial clean/YAGNI. Unconditional: policy numbers.

## Kill (V15.6)

| Fix | Where |
|-----|--------|
| Honesty: Lean LOC = size M; semantic quality = J | `USER-RULES.paste.txt` + `option-c-core.mdc` |
| NATIVE LEAN / lean meter wording: finite size ≠ ∀ quality | paste Block 3 + companions |
| Ponytail force vs persuasion explicit | `project-rules/ponytail.mdc`, skill |
| Doctrine / chain / README / RELEASE | this file + breakthrough chain |

No complexity / must-test meters — still finite and gameable; residual stays A2.

## Residual (honest A2)

LOC is a size proxy. Extra meters (cyclomatic, “test required”) remain finite
and bypassable. Soft ladder may still yield; agent can still emit legal mediocre
diffs under the size roofs.

## Related

- [`LEAN-VERNACULAR-FORCE-PSTAR.md`](LEAN-VERNACULAR-FORCE-PSTAR.md) (P*-9/10)
- [`MECHANICAL-INCOMPLETENESS-PSTAR.md`](MECHANICAL-INCOMPLETENESS-PSTAR.md)
- [`SOFT-FORCE-SCHISM-PSTAR.md`](SOFT-FORCE-SCHISM-PSTAR.md) (P*-14)
- [`BREAKTHROUGH-CHAIN.md`](BREAKTHROUGH-CHAIN.md)
