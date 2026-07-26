# P* — ACT-install schism (U-curve action-class paradox)

Finished unconditional counterexample. Closed in V11.0.1.

## Verdict

Block 1 labeled lockfile-faithful install as ACT-now while Block 3 and the
live install hook forced ask on every syntactic realization (including
`npm ci`). Green meters encoded the recency/hook branch. The advertised ACT
class was unreachable by construction.

## Strategy

Diff Block 1 ACTION CLASSES against Block 3 MECHANICAL GATES and the hook
predicate for the same verb. Where primacy grants ACT and recency/force
always ask with no carve-out, the class is a dead claimed path — especially
when proof evals green the contradictory branch.

## Claim (C)

ACTION CLASSES are coherent and executable: lockfile-faithful install with
unchanged dependency graph is ACT-now; ASK ONCE covers only untrusted
installs; Block 3 / hooks implement that split; ACT defect axioms remain
satisfiable.

## Instance (P*)

1. Primacy: ACT for lockfile-faithful install (`USER-RULES.paste.txt` pre-kill).
2. Primacy: ASK ONCE for untrusted installs — intentional split.
3. Recency: Install gate (ask) — no lockfile exception.
4. Hook: `ask-gated-shell.sh` matches `npm ci` / install / `pip install` →
   always `permission:ask`.
5. Meter: `_proof_evals.py` asserts install → ask; never `npm ci` → allow.

| Branch | Violates |
|--------|----------|
| Run `npm ci` as ACT-now | Recency + hook ask |
| Agent chat-confirms first | ACT + “asking ACT is a defect” |
| Stop / rewrite per hook-block escape | Refusing clear ACT; no install rewrite escapes the regex |

## Failure (by construction)

Primacy ↔ recency ↔ registered hook disagree on one predicate. Force and
meters pick ask. ACT lockfile path is dead letter. Independent of model,
network, or flake. Hook cannot decide “this task did not change the
dependency graph,” so a syntactic ACT carve-out would also break the
sequence rule (manifest edit then install gated).

## Kill (V11.0.1)

1. Delete ACT lockfile carve-out from ACTION CLASSES.
2. ASK ONCE for package installs (lockfile-faithful ci included).
3. Keep Install gate (ask) + `ask-gated-shell.sh` universal install ask.
4. Proof: `npm ci` → ask; PROOF-EVALS row; P* recorded.
5. Sequence rule unchanged (still gated at install after manifest edit).

## Residual (named, not closed)

MCP / non-shell remote publish still channel-incomplete (sibling of dead-gate
residuals). Surgical `rm -rf <file>` still asks while `rm <file>` allows.
