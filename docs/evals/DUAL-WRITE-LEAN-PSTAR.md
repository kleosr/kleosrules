# P* — Dual-Write Lean Bypass (P*-17)

Finished unconditional counterexample. **Killed in V16.0.12.**

## Verdict

Lean/vernacular/prose roofs on `Write`/`StrReplace` did not apply to Shell
CODE_EXT writes. Comment-free oversize heredoc allowed while Write denied.
Opaque ask was emptied (`opaque_write_ask_message: ""`).

## Claim (C) — pre-fix

Force-first Native Lean: oversized CODE_EXT cannot land under green TOOLCHAIN
except via named A2 residuals (template-prose FN, shell-argv comment smuggle,
lexical ask-scope).

## Instance (pre-kill)

1. Body B = 130 LOC comment-free `.ts`.
2. Write B → lean deny.
3. Shell `cat > path <<'END'` + B → allow.
4. `tee` / `sed -i` / `git apply` → allow (empty opaque message).

## Kill (V16.0.12)

| Fix | Where |
|-----|--------|
| Parse heredoc/redirect embedded body → prose + vernacular + lean | `engine/shell.rs` |
| Restore opaque ask; treat `<<` / `>` CODE_EXT as opaque when body not checked | `shell.rs` + `policy/shell.json` |
| Re-enable ask-scope (P*-16 sibling) | `policy/ask-scope.json` |
| Meters | `tests/integration.rs`, `fleet/bench.rs` |

## Residual (honest A2)

Shell writes whose payload is not in argv (external file + `tee`/`sed -i`)
still **ask** (not auto-lean). Agent can confirm ask. Template prose FN and
Rice quality remain.

## Related

- [`ANTI-DRIFT-DRIVE-BY-PSTAR.md`](ANTI-DRIFT-DRIVE-BY-PSTAR.md) (P*-12)
- [`LEAN-SIZE-QUALITY-PSTAR.md`](LEAN-SIZE-QUALITY-PSTAR.md) (P*-15)
- [`BREAKTHROUGH-CHAIN.md`](BREAKTHROUGH-CHAIN.md)
