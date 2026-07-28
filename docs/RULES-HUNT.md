# Rules hunt timeline — kleosr V13 → V16

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
V15.5 (soft-force honesty)
  P*-14 KILL: soft = J-authority when routed; skill Self-target pause;
    README slogan scoped — chat remains ungated by design
V15.6 (lean size ≠ quality)
  P*-15 KILL: lean meter = size M roofs; semantic quality / clean / YAGNI = J;
    no new complexity meters (still finite / gameable)
V16 (kleosr — shell zero / fleet CLI in kleos-gate)
  Pack identity kleosr; tooling Rust-only; vernacular SSOT in project-rules/
V16.0.12
  P*-17 KILL: dual-write lean on Shell heredoc + ask-scope re-enable
V16.0.13
  P*-16 KILL: absolute `file_loc_max` — per-event lean does not compose to
    state bound (staircase composition)
V16.0.17–18
  Lab fluid: shell deny empty; opaque Shell CODE_EXT deny; only rm ASK
V16.0.19
  P*-18 KILL: Native Delete extensionless / bare roots (`delete.json`);
    ask-scope restore; Life OS brand retired; honesty ↔ lab reconcile
```

## Status

P*-13…P*-18 deepened and resolved. Mechanism A2 residuals remain (incl.
delete.rs sequential-singles sibling of P*-16). Implementation: kleosr Master
Mind V16.0.19 — see `docs/RELEASE.md`, `docs/evals/DELETE-STAIRCASE-PSTAR.md`,
`user-rules/USER-RULES.paste.txt`.

Fictional “V14 Rust-Core” paste in older drafts is **superseded** — pack V14
was Python lean/vernacular; Rust is **V15**. Shell fleet tooling retired in **V16**.
