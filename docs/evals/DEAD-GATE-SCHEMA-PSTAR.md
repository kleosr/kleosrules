# P* — Dead gate via event schema drift (secrets / prompt)

Finished unconditional counterexample. Closed in V10.1.18.

## Verdict

A MUST-NEVER gate can be registered with `failClosed: true`, pass every
self-meter, and still be structurally unable to block — when its JSON
vocabulary is not the vocabulary the platform event reads.

## Strategy

Assert hook output against **event contract**, not against the hook’s own
dialect. Live block field + exit 2; drop registrations on events with no
output fields.

## Claim (C)

If a MUST-NEVER class has a registered failClosed gate and meters are green,
the platform enforces that class on every registered surface.

## Instance (P*)

1. `hooks.json`: `block-secrets.py` on `beforeSubmitPrompt`, failClosed true.
2. Hook detects secret; emits `permission: deny` (+ agent_message) with exit 0.
3. Platform contract for `beforeSubmitPrompt`: `{continue, user_message}` —
   does not read `permission`.
4. `failClosed` only on crash / timeout / invalid JSON — success path skips it.
5. Prompt with secret is submitted. MUST-NEVER “no secret material” has no live
   gate on that surface.

Same class: `scan-edited-file-for-prose.py` on `afterFileEdit` /
`afterTabFileEdit` emitting deny/follow-up while those events honour no
output fields — inert, not reactive.

## Failure (by construction)

Schema mismatch between emitter and event. Independent of model, prompt,
U-curve, or agent error. Meters that assert `permission == deny` only validate
the script’s private dialect, not platform enforcement.

## Kill (V10.1.18)

1. `block-secrets.py` branches on `hook_event_name`: `continue:false` + exit 2
   on `beforeSubmitPrompt`; `permission:deny` + exit 2 on tool events.
2. Remove inert `afterFileEdit` / `afterTabFileEdit` registrations.
3. `_proof_evals.py` asserts prompt-secret + write-secret with event vocabulary.
4. New meter `_verify_hook_contracts.py`: UNKNOWN-EVENT, SCHEMA-DRIFT,
   DEAD-GATE, NO-OP-DECISION, UNENFORCED-VERB, UNCOVERED.
5. Mutation: reintroduce permission-deny on prompt → DEAD-GATE + SCHEMA-DRIFT;
   `_selftest` alone stays blind.

## Residual (named, not closed)

`truncate` empty DB, native `Delete` tool (non-shell), matcher drift if
platform matchers diverge from declared Write|StrReplace|EditNotebook.
Next measurement: TTD/TTR in production, not another prose layer.
