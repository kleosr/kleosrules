# Rules hunt timeline — V13 → V15.4

## Structural timeline

```
V13 (Python hooks, contract initial)
  P*-1..P*-6 against V13
V14 (lean_meter.py + vernacular force; still Python)
  P*-7..P*-10 KILLED by V14
  P*-11 substrate lock-in (bidirectional)
  P*-12 ask-scope forceless
V15 (Rust-core + zero policy hardcode)
  P*-11 KILL: kleos-gate + policy JSON
  P*-12 KILL: ask-scope ledger
  A2 mechanism residuals named
V15.4 (contract language)
  P*-13 KILL: MUST-NEVER/M vs /J; honesty table; scoped distrust;
    verified-intent repair; J sovereign override
  Zero Rust/policy schema change — language matches main.rs force surface
```

## Status

P*-13 deepened and resolved (A3 language kill). Mechanism A2 residuals remain.
Implementation: Master Mind V15.4 — see `docs/RELEASE.md`,
`docs/evals/PERFORMATIVE-TRILEMMA-PSTAR.md`, `user-rules/USER-RULES.paste.txt`.

Fictional “V14 Rust-Core” paste in older drafts is **superseded** — pack V14
was Python lean/vernacular; Rust is **V15**.
