# Handoff

**Goal:** Land P*-16 Staircase Composition kill (absolute lean file roof) and persist epistemic memory.

**Status:** done (mechanism + docs); open sibling remains

## Done

- Absolute `file_loc_max: 700` on Write + projected StrReplace — evidence: `cargo test --bins -- lean::` (5/5); live staircase deny at 801 / final 601
- Honesty table split increment vs absolute — `user-rules/USER-RULES.paste.txt` + `option-c-core.mdc`
- Eval + chain — `docs/evals/STAIRCASE-COMPOSITION-PSTAR.md`; `docs/evals/BREAKTHROUGH-CHAIN.md`; pack `16.0.13`
- Integration + release binary — `cargo test` 30/30; `gate-diff` GATE_DIFF_PASS; `hooks/bin/kleos-gate` rebuilt
- Vault — [[wiki/projects/kleosr/Sessions/2026-07-27-pstar16-staircase]], Decisions, Learnings, hot, log

## Open

- [ ] Kill `delete.rs` sequential single Deletes ≡ mass Delete (same event/state quantifier class)
- [ ] Re-paste User Rules / `kleos-gate install` so live Cursor loads V16.0.13 paste+policy
- [ ] Optional: `.ipynb`/`.tf`/`.jl` in `code_extensions` (FP tradeoff)

## Blockers

- none

## Next

Audit and kill the `delete.rs` sequential-singles sibling of P*-16 (same class).

## Verify

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
cp -f target/release/kleos-gate ../bin/kleos-gate
hooks/bin/kleos-gate gate-diff
hooks/bin/kleos-gate check-user-rules
```

## Notes

- Model / chat: Cursor Grok 4.5 — P*-16 land + Obsidian write-back flush
- Related skills: obsidian-memory, session-handoff, unconditional-counterexample
- Vault SSOT: `/home/kleosr/rootsidian/kleosr` — never wipe; persist INTO wiki/
