# P* — Hardcoded execution schism / substrate lock-in (P*-6 / P*-11)

Finished unconditional counterexamples. Closed in V15.0.0 (finite instruments).

## Verdict

V13 tied shell JSON parse to `python3`. V14 deepened the lock-in by adding
quality instruments (`lean_meter.py`, vernacular force) on the same Python
enforcement substrate. Migrating to `.sh`/Rust without extracting policy
orphaned both sides. V15 replaces the hot path with a Rust engine and moves
policy to JSON files.

## Claim (C)

Master Mind V14 satisfies, by construction, gates migratable off Python without
collapsing enforcement or V14 quality instruments.

## Instances

### P*-6 HARDCODED EXECUTION SCHISM (V13 floor)

Contract: *Shell JSON via python3 — deny if python3 cannot run.* vs user ask to
drop Python. Unidirectional dependency.

### P*-11 ENFORCEMENT-SUBSTRATE LOCK-IN (bidirectional)

V14 quality meters implemented in Python on Python gates; cosmetic `.sh`
wrappers still invoked `python3` for JSON. Migrate enforcement → orphan lean /
vernacular callers; migrate instruments → orphan `gate-write`; remove python3 →
collapse shell parse.

## Kill (V15)

- `hooks/kleos-gate` Rust binary on Cursor hook hot path (`hooks.json`)
- `hooks/policy/*.json` — shell / lean / secrets / ask-scope (no policy hardcode in `.rs`)
- User Rules: drop python3 parser roof; document Rust + policy
- Residual: cargo/binary availability is a new named substrate (not ∀ freedom)

## Related

- [`ANTI-DRIFT-DRIVE-BY-PSTAR.md`](ANTI-DRIFT-DRIVE-BY-PSTAR.md)
- [`LEAN-VERNACULAR-FORCE-PSTAR.md`](LEAN-VERNACULAR-FORCE-PSTAR.md)
