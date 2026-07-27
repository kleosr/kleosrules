# P* — Staircase Composition (P*-16)

Finished unconditional counterexample. **Killed in V16.0.13.** Verified against
shipped `hooks/bin/kleos-gate` pre-fix: every per-event check returned the
correct answer on its own input — not a heuristic false negative.

## Verdict

The lean meter bounded **per-event increments**. The honesty table advertised a
roof on **size**. A per-event monotone bound composes to no bound at all, so
`sup(file LOC | every event exit 0)` was unbounded while the contract read
`Total (proven)`. Defect is quantifier scope, not detection.

## Claim (C) — pre-fix

`user-rules/USER-RULES.paste.txt` honesty table:

| Surface | Class | Enforcement | Guarantee |
|---|---|---|---|
| Lean LOC caps (size only) | M | Gate binary | Total (proven) |

Scoped to new-file / net-delta roofs. P*-15 attacked the quality axis and
**conceded the size axis as total and proven**.

## Claim (C′) — post-fix

Lean M enforces (a) per-event increment roofs **and** (b) an absolute post-state
file roof (`file_loc_max`). (b) is closed under composition; (a) alone is not.
Any M row whose property is cumulative over events must be checked against
projected post-state, never against delta alone.

## Instance (P*)

Policy constants mutually inconsistent: `net_delta` (200) > `new_file_loc` (120),
so the weaker roof governed every event after the first.

1. `Write src/x.ts`, 1 line → path absent → allow; file now exists.
2. `Write src/x.ts`, 201 lines → `delta = 200` → allow. 120 roof beaten by 81.
3. Event *k* writes `1 + 200(k−1)` lines → `delta = 200` ∀*k* → allow.
   Any target *N* in `⌈(N−1)/200⌉ + 1` events.

Pre-fix live: one-shot 121/201 deny (gate live); staircase 1→201→…→1001 all
allow. Final 1001 LOC — 8.3× advertised roof.

## Failure (by construction)

- Evidence: `hooks/policy/lean.json`; `lean.rs` path-absent / saturating_sub /
  StrReplace delta. Meter imports `policy`, `env`, `fs`, `Path` — **not**
  `ledger`, so no cross-event state by construction.
- Every event satisfies its own roof; composition satisfies no roof.
- Why ¬C: C asserts a *total, proven* size roof quantified over events, not
  states.

### Meta-defect

Zero unit tests in `lean.rs`; zero integration coverage of `lean::` /
`new_file_loc` / `net_delta` / `KLEOS_LEAN`. Only M row labelled
`Total (proven)` with no executable proof — `cargo test` could not fail on
any lean property.

## Kill (V16.0.13)

| Fix | Where |
|-----|--------|
| `file_loc_max: 700` + env | `hooks/policy/lean.json` |
| Policy fields | `src/policy.rs` |
| Post-write + projected post-replace absolute roof | `src/engine/lean.rs` |
| 5 unit tests incl. staircase regression | `src/engine/lean.rs` |
| Honesty table: split increment vs absolute | `USER-RULES.paste.txt` |
| Doctrine: per-event ≠ state bound | paste + option-c |

700 lifted from ponytail’s hard limit (J → M). Check is
`line_count(post_state) > constant` — zero soft surface added.

Measured: `sup(file LOC | all events green)` ∞ → 700.

## Why prior residuals were insufficient

| Named residual | Why it does not cover P*-16 |
|---|---|
| A2 template-prose FN | Detection FN — here every answer is right |
| A2 shell-argv smuggle | Parser blind spot — no parser involved |
| A2 lexical ask-scope | Token heuristic — no heuristic involved |
| P*-13 M/J split | Language kill; this is syntactic M, correctly implemented, still ¬C |
| P*-15 lean size ≠ quality | Explicitly conceded size axis as total/proven |
| LOCAL-GLOBAL-COMPOSITION | Semantic sibling; P*-16 is one mechanical predicate over time |

## Sibling gaps (open — same class)

| Gap | Status |
|---|---|
| N sequential single-path Deletes ≡ one mass Delete (`delete.rs` `paths.len() > 1`) | Same quantifier bug, second site |
| `.ipynb` / `.tf` / `.jl` absent from `code_extensions` | Extension allowlist; FP tradeoff if added |
| `check_content_path_vernacular_denies` non-hermetic | Environment-dependent — not unconditional P* |

## Related

- [`LEAN-SIZE-QUALITY-PSTAR.md`](LEAN-SIZE-QUALITY-PSTAR.md) (P*-15)
- [`PERFORMATIVE-TRILEMMA-PSTAR.md`](PERFORMATIVE-TRILEMMA-PSTAR.md) (P*-13)
- [`LOCAL-GLOBAL-COMPOSITION-PSTAR.md`](LOCAL-GLOBAL-COMPOSITION-PSTAR.md)
- [`DUAL-WRITE-LEAN-PSTAR.md`](DUAL-WRITE-LEAN-PSTAR.md) (P*-17)
- [`BREAKTHROUGH-CHAIN.md`](BREAKTHROUGH-CHAIN.md)
