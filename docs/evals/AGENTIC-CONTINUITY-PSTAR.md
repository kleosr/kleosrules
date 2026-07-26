# P* — Agentic continuity (session freshness + normalize + injection)

Finished unconditional counterexample. Closed in V11.2.0.

## Verdict

Policy claimed session verify ordering and freeze-loop detection; hooks only
counted aggregate edits/verifies on a mutable ledger. Subagent briefs and MCP
tool output carried no injection frame scan. Write gate denied prose without
normalize transform. Shell gates split across four bash/python entrypoints.

## Strategy

Append-only event log per conversation (`hookio.append_event` / `freshness`).
Normalize mode on gate-write (`updated_input` via `strip_prose`). Injection
frames in `injection_lib` for untrusted surfaces. Merge shell into
`gate-shell.py`. Wire subagentStart/Stop, postToolUseFailure, sessionStart,
preCompact. CI mirror `scripts/gate-diff.py`.

## Claim (C)

Session continuity, injection notice, normalize transform, and shell verdict
parity are live on advertised events with gauntlet proof.

## Instance (P*)

1. `session-ledger` incremented `edits`/`verifies` counters — verify-after-edit
   ordering invisible; stop hook used stale counts.
2. Repeat identical deny → agent freeze loop; no `deny_repeat` escalation.
3. Subagent briefs with force-push / override frames not gated at spawn.
4. Tool/MCP output injection frames not surfaced to agent context.
5. Prose write only denied — no `strip_prose` / `updated_input` path.
6. Four shell hooks — drift risk; jq-dead category was killed but split remains.
7. No CI prose scan; no pre-commit mirror.

## Failure (by construction)

Agent could claim Done after verify-before-edit; retry blocked writes;
inherit injection via MCP; subagent spawn with destructive brief.

## Kill (V11.2.0)

1. Event log + `freshness()` verify ordering; stop uses `unverified` paths.
2. `gate-write` normalize (`KLEOS_NORMALIZE`); repeat deny fingerprint +
   `deny_repeat` event.
3. `injection_lib` frames + actions; session-ledger injection notice.
4. `gate-subagent` deny gated briefs; subagentStop → parent edit events.
5. `gate-fail` records deny fingerprints on `permission_denied`.
6. `session-boundary` sessionStart roof + unverified carry-over; preCompact dirty paths.
7. `gate-shell.py` merged shell verdict; hooks.json single entry.
8. `scripts/gate-diff.py` + pre-commit installer; `.github/workflows/gates.yml`.

## Residual

Injection regex false positives/negatives; normalize unsafe inline block
comments return None (deny); Rice semantic Done; postToolUse event volume;
platform may not pass all hook payload fields — inferred schemas.
