# P* — Anti-drift / drive-by forceless (P*-12)

Finished unconditional counterexample. Closed in V15.0.0 (finite ask-scope gate).
Re-enabled after V16.0.6 ACT demotion — see V16.0.12 / P*-17 sibling.

## Verdict

V14 gates inspect content (comments/secrets/vernacular), danger (shell), and
volume (lean). None compare user ask ↔ touched paths. Drive-by edits pass all
gates by construction. V15 adds a policy-driven ask-scope ledger (heuristic).

## Claim (C)

Master Mind V14 satisfies, by construction, no context-drift and no extra
drive-by code.

## Instance

### P*-12 MISSING GATE ON SEMANTIC ASK-SCOPE

User: rename in `auth.ts`. Agent also edits `utils.ts`. `gate-write` allows
(no comments, vernacular OK, under lean caps). Soft text “inside the ask” has
no gate.

## Kill (V15)

- `hooks/policy/ask-scope.json` + Rust ask-scope engine
- Prompt path tokens recorded; Write/StrReplace paths outside allow set →
  ask or deny per policy
- Residual: ask↔diff alignment is undecidable in full (Rice); ledger is finite

## Related

- [`HARDCODED-EXECUTION-SCHISM-PSTAR.md`](HARDCODED-EXECUTION-SCHISM-PSTAR.md)
