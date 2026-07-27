# Rules hunt timeline — V13 → V15

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
  A2 residuals named
```

## Status

Hunt closed (Deepened / A2). Implementation: Master Mind V15 — see
`docs/RELEASE.md`, `hooks/kleos-gate/`, `hooks/policy/`.

Fictional “V14 Rust-Core” paste in older drafts is **superseded** — pack V14
was Python lean/vernacular; Rust is **V15**.
