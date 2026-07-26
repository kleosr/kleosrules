# P* — Green-proof inversion (jq-dead gates + unwired surfaces)

Finished unconditional counterexample. Closed in V11.1.0.

## Verdict

The policy layer claimed live roofs; the weakest meter (`_selftest.py`) stayed
green while jq-dependent shell gates were dead without python3, content
matchers were guesses, and read/MCP/Delete surfaces were unwired. Fixing the
proof layer and wiring those surfaces is the kill — not more essay.

## Strategy

Execute every claimed gate against adversarial payloads. Prefer parser
hard-deps already required by the pack (`python3`). Meter registration
coverage, matcher tripwires, and failClosed parser-missing as deny.

## Claim (C)

Green TOOLCHAIN / selftest implies shell, write, read, MCP, and Delete roofs
are live on every advertised channel.

## Instance (P*)

1. `ask-gated-shell.sh` / `deny-danger.sh` parsed stdin with `jq` under
   `set -euo pipefail`; `jq` undeclared in `install.sh`.
2. Missing parser → exit 127 / empty stdout; with `failClosed` freezes shell;
   `block-dangerous-git.sh` allowed when python3 missing.
3. `_selftest.py` never exercised those shell gates → green while dead.
4. `preToolUse` matcher `Write|StrReplace|EditNotebook` guessed tool names.
5. Prose gate missed `edits[]`, `patch`, `.mts`, `#`/ `--` dialects.
6. Shell missed base64 / `sed -i` / `git apply` channels.
7. No `beforeReadFile`, `beforeMCPExecution`, or native `Delete` gate.

## Failure (by construction)

Meters certified a harness that did not enforce its own roofs on missing
deps and missing events. Independent of model quality.

## Kill (V11.1.0)

1. Shell gates: python3 JSON parse; deny if python3 cannot run.
2. `gate-write.py` (no matcher): secrets + prose + vernacular; walk_strings.
3. `gate-read.py`, `gate-mcp.py`, `gate-delete.py` (Delete matcher + route).
4. Shell opaque → ask; base64 decode → deny when prose.
5. Project hooks: `hooks.project.json` + `scripts/sync-hooks-to-repos.sh`.
6. `session-ledger.py` / `stop-verify.py`; `obedience-report.py`.
7. `_gauntlet.py` carries P* regressions; `_verify_hook_contracts` adds
   UNPROBED-MATCHER + new event tripwires.

## Residual

Rice / semantic Done; stop follow-up ≠ hard block; Delete payload schema
inferred; postToolUse spawn cost; prompt-type hooks unused.
