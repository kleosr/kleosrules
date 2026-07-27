# P* — Verify-surface mismatch (force credibility)

Finished unconditional counterexample. Closed in V13.0.0.

## Verdict

V11.2 freshness ordering was live, but the VERIFY recognizer did not match
the pack’s natural agent argv (`python3 hooks/_gauntlet.py`). Stop fired
forever after real green gauntlets — worse than no gate (force credibility
collapse).

## Strategy

Instrument string surface = evidence. Match natural agent commands, not only
command-head bare tokens. Deduplicate identical stop followups. Narrow
failClosed write matcher blast. Fingerprint project hooks in verify-sync.

## Claim (C)

Session verify is mechanical: a house-gauntlet Shell command the agent
normally runs clears dirty edits; stop only fires when evidence cannot cover
the work; force stays scarce and credible.

## Instance (P*)

```
VERIFY.search("python3 hooks/_gauntlet.py")  → False   (V11.2)
VERIFY.search("pytest")                      → True
```

Dirty paths stuck; stop re-stated the same three files every turn after
`ALL_GAUNTLET_PASS` in chat.

## Failure (by construction)

Human-satisfying Martin evidence ≠ machine verify event. Transformers treat
chat-cited green as Done; sticky false stop trains ignore-harness.

## Kill (V13.0.0)

1. `session-ledger` VERIFY matches `python3 …/hooks/_gauntlet.py` and pack
   verify scripts anywhere in the argv.
2. `stop-verify` suppresses repeat followup for unchanged dirty fingerprint.
3. `gate-write` matcher Write|StrReplace|EditNotebook — Shell cannot freeze
   on import crash.
4. `verify-sync` fingerprints project hooks; scan/install runs
   `sync-hooks-to-repos.sh`.

## Residual

Rice semantic Done; not every possible Shell phrasing; platform payload
schemas inferred.
