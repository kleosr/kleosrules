# Release — v15.3.0

## V15.3.0 — Python zero in pack

1. Delete remaining pack Python: `scripts/gate-diff.py`, `scripts/obedience-report.py`,
   `lib/check-user-rules.py`.
2. Rust CLI on `kleos-gate`: `gate-diff`, `obedience-report`, `check-user-rules`.
3. Pre-commit installer → `kleos-gate gate-diff`. Scrub live docs/AGENTS of python3 recipes.
4. Residual: `.py` remains only as CODE_EXT in `lean.json` (gate foreign app code); historical P* prose may mention past Python kills.

## V15.2.0 — pre-flight --check-content

1. `kleos-gate --check-content` (+ optional `--path`): agent self-check before Write.
2. Contract PRE-FLIGHT: Cursor hook is backstop; fix until pre-flight exit 0.
3. `scripts/benchmark-hooks.sh` replaces `benchmark-hooks.py` (jq + time).
4. Residual: pre-flight is agent discipline; Cursor wire-up still required as roof.

## V15.1.0 — retire Python proof substrate

1. Port house meters to `hooks/kleos-gate/tests/integration.rs` (`cargo test`).
2. Delete legacy `hooks/*.py` gates/meters and python3 shell wrappers.
3. TOOLCHAIN / CI / USER-RULES Proof line = `cargo test -p kleos-gate` only.
4. Residual: `lib/check-user-rules.py` optional sqlite probe (not enforcement).

## V15.0.0 — Rust kleos-gate + ask-scope

1. P*: `docs/evals/HARDCODED-EXECUTION-SCHISM-PSTAR.md` (P*-11),
   `docs/evals/ANTI-DRIFT-DRIVE-BY-PSTAR.md` (P*-12).
2. Kill: Rust `hooks/bin/kleos-gate` hot path; `hooks/policy/*.json`; no
   python3 in `hooks.json`; fail-closed missing policy; ask-scope ledger.
3. Residual: cargo/binary substrate; ask↔diff heuristic; Rice Done.
   Interim Python proof closed in V15.1.0.

## V14.0.0 — lean meter + vernacular force

1. P*: `docs/evals/LEAN-VERNACULAR-FORCE-PSTAR.md` — ponytail ladder forceless;
   vernacular machine fields / topology dead vs gate.
2. Kill: `lean_meter.py` (new-file / net-LOC caps, `KLEOS_LEAN`); vernacular
   `pack_native`, `boolean_prefixes`, `allowed_path_prefixes`,
   `forbidden_class_suffixes`; companion/skill soft-vs-roof wording; gauntlet
   P16/P17.
3. Residual: Rice Done; taste of “organized”; import/visibility soft; meters
   gameable / finite.

## V13.0.0 — instrument surface + force credibility

1. P*: `docs/evals/VERIFY-SURFACE-PSTAR.md` — VERIFY regex missed
   `python3 hooks/_gauntlet.py`; sticky stop; universal gate-write freeze;
   project hooks outside verify-sync.
2. Kill: argv-aware `is_verify_command`; stop followup dedupe; gate-write
   matcher Write|StrReplace|EditNotebook; verify-sync hook fingerprint;
   scan/install run `sync-hooks-to-repos.sh`.
3. Residual: Rice Done; novel Shell phrasings; platform payload schemas.

## V11.2.0 — agentic continuity

1. P*: `docs/evals/AGENTIC-CONTINUITY-PSTAR.md` — session ledger counters vs
   verify ordering; no normalize transform; injection in MCP/subagent; split shell.
2. Kill: append-only event log + `freshness()`; `gate-write` normalize +
   repeat-deny escalation; `injection_lib`; `gate-shell` merge; subagent/session
   boundaries; `gate-fail`; CI `gate-diff.py` + GitHub `gates.yml`.
3. Residual: injection regex limits; normalize unsafe inline blocks → deny;
   Rice semantic Done.

## V11.1.0 — green-proof inversion kill

1. P*: `docs/evals/GREEN-PROOF-INVERSION-PSTAR.md` — green selftest while
   jq/shell gates dead; matcher/payload/MCP/read/Delete holes.
2. Kill: python3 shell parse (deny if missing); gate-write/read/mcp/delete;
   opaque shell ask; project hooks sync; session-ledger + stop-verify;
   `_gauntlet.py` + UNPROBED-MATCHER; User Rules GATE MANIFEST + ASK ROUTING
   + secret-read roof.
3. Residual: Rice; stop follow-up ≠ hard block; Delete schema inferred.

## V11.0.1 — ACT-install schism kill

1. P*: `docs/evals/ACT-INSTALL-PSTAR.md` — primacy ACT lockfile install vs
   recency Install gate + hook ask on `npm ci` (dead claimed ACT path).
2. Kill: drop ACT lockfile carve-out; ASK ONCE for all package installs
   (ci included); keep Install gate + ask-gated-shell; proof `npm ci` → ask.
3. Why not hook carve-out: force cannot know “this task did not change the
   dependency graph”; sequence rule still gates post-manifest install.

## V11.0.0 — mechanical incompleteness (Rice ceiling)

1. P*: `docs/evals/MECHANICAL-INCOMPLETENESS-PSTAR.md` — finite gauntlet
   cannot guarantee absolute semantic correctness (deadlock∀ class).
2. Kill: success = low entropy + named residual; green ≠ zero-margin ∀.
   Topology / silent / closed-loop survive as probabilistic quality.
3. Protocol entropy for absolute pre-contact Done is maxed; remaining
   progress is reality-loop. User Rules IDENTITY + PRODUCT GAUNTLET; agent;
   VERIFICATION-CHAIN layer 6.

## V10.1.18 — dead-gate schema kill (secrets + contract meter)

1. P*: `docs/evals/DEAD-GATE-SCHEMA-PSTAR.md` — failClosed + green meters ≠
   live block when output vocabulary ≠ event contract.
2. Kill: `block-secrets.py` event-branch; drop inert after*FileEdit regs;
   `_verify_hook_contracts.py`; proof-evals prompt/write secret asserts.
3. Mutation: wrong schema → DEAD-GATE; TOOLCHAIN + MECHANICAL GATES updated.

## V10.1.17 — deterministic–plastic incompleteness (3-layer stack)

1. P*: `docs/evals/DETERMINISTIC-PLASTIC-PSTAR.md` — prompt-alone cannot be
   fully plastic and fully deterministic.
2. Kill: soft/roof partition + never-fight-deny (no freeze) + epistemic
   memory; false “must comment” → DEBT/test.
3. `AGENTIAL-CONTROL.md` = three layers; IDENTITY / SOFT VS ROOF / agent.mdc.

## V10.1.16 — executable epistemology + formalization barrier

1. Prefer path: `docs/EXECUTABLE-EPISTEMOLOGY.md` (types/asserts/tests).
2. P*: `docs/evals/FORMALIZATION-BARRIER-PSTAR.md` — not all intent is
   efficiently executable; false dilemma Type Hell vs silence.
3. Kill: non-formalizable → durable prose outside app AST; roof NO COMMENTS
   unchanged. EPISTEMIC-PERSIST + User Rules + agent.mdc updated.

## V10.1.15 — relational verification + reality-loop

1. P*: `docs/evals/LOCAL-GLOBAL-COMPOSITION-PSTAR.md` — local∧local ⇏ global.
2. Docs: `RELATIONAL-VERIFICATION.md`, `REALITY-LOOP.md`, `VERIFICATION-CHAIN.md`.
3. User Rules: IDENTITY honesty; LOOP RELATIONAL + REALITY-LOOP; agent.mdc;
   soft PROOF-EVALS. No invent ops theater; TTD/TTR or unknown.

## V10.1.14 — epistemic black hole kill (persist ≠ comments)

1. P*: `docs/evals/EPISTEMIC-BLACK-HOLE-PSTAR.md` — silent code + ephemeral
   chat ⇒ future O(N) reconstruct / Lost-in-the-Middle.
2. Kill: `docs/EPISTEMIC-PERSIST.md` — types/tests or durable O(1) docs;
   SYSTEM INTEGRITY #5; NO COMMENTS clarification; agent.mdc.
3. Decoupling + closed-loop companions updated; soft PROOF-EVALS rows.

## V10.1.13 — closed-loop coupling (graph integrity)

1. Document: `docs/CLOSED-LOOP-COUPLING.md` — no hanging wires, atomic
   passes, flow trace, negative entropy.
2. User Rules LOOP SYSTEM INTEGRITY; `agent.mdc`; soft PROOF-EVALS rows;
   DOC MAP / MODEL-SPEC / README.

## V10.1.12 — epistemic resonance (brownfield loop)

1. Document: `docs/EPISTEMIC-RESONANCE.md` — cartography, private-match,
   blast radius; absolute quality equation.
2. User Rules NATIVE LEAN + LOOP; `agent.mdc` QUALITY; DOC MAP / MODEL-SPEC /
   README links.

## V10.1.11 — cognitive decoupling (silent transmutation)

1. Document: `docs/COGNITIVE-DECOUPLING.md` — Phase A blueprint / Phase B
   silent code; semantic reflection; intelligence shield at primacy.
2. User Rules: IDENTITY entropy line; NO COMMENTS two-phase; NATIVE LEAN
   names-as-comment; DOC MAP link.

## V10.1.10 — topological User Rules (U-curve + recurrence)

1. Document: `docs/TOPOLOGICAL-PROMPT.md` (Lost-in-the-Middle U-curve;
   primacy / mid / recency; hook recurrence anchors).
2. Reorder `USER-RULES.paste.txt`: Block 1 roof, Block 2 dictionary,
   Block 3 execution gates at tail. Soft Defaults / PRECEDENCE in primacy.

## V10.1.9 — deterministic cognitive collapse (IME CoT)

1. Document: `docs/COGNITIVE-COLLAPSE.md` — four CoT rewrites → biological
   transpiler (universal Transformer defect attack).
2. Links from DEFECT-COMPENSATION, MODEL-SPEC, AGENTIAL-CONTROL, README;
   NATIVE LEAN pointer.

## V10.1.8 — defect compensation (CoT / entropy / tools)

1. Document: `docs/DEFECT-COMPENSATION.md` — cognitive asymmetry, entropy
   restrictor, stateful-tools / stateless-brain loop.
2. Links from MODEL-SPEC, AGENTIAL-CONTROL, README; NATIVE LEAN pointer.

## V10.1.7 — agential control (persuasion + force)

1. Document dual stack: `docs/AGENTIAL-CONTROL.md` + MODEL-SPEC link.
2. Keep V10.1.6 scoped Defaults (roofs ≠ soft). Soft persuasion on taste +
   hook force on roofs is intentional; soft Defaults never waive gates.
3. Pointers in User Rules ALWAYS-ON and `agent.mdc`.

## V10.1.6 — kill Defaults vs religion P*

1. Record P*: `docs/evals/DEFAULTS-RELIGION-PSTAR.md`.
2. Scope soft craft defaults ≠ roof/gate/pass-what-may/never-fight-deny.
   `agent.mdc`, `native-lean-autoload.mdc`, `ponytail.mdc`, `lean-code.mdc`,
   User Rules ALWAYS-ON, ponytail skill Off line.

## V10.1.5 — kill precedence paradox P*

1. Record P*: `docs/evals/PRECEDENCE-PARADOX-PSTAR.md`.
2. Soft rules ≠ MUST-NEVER. Gates implement roof; never outrank it.
   PRECEDENCE: MUST-NEVER first including over gates; gates outrank only
   soft (non-MUST-NEVER) policy. Gate allow ≠ waiver.

## V10.1.4 — kill Martin gauntlet P*

1. Record P*: `docs/evals/MARTIN-GAUNTLET-PSTAR.md`.
2. Close Done tier-3 silent ship: no house gauntlet + land code → ASK ONCE
   (accept-no-gauntlet-risk or wire verify). PRODUCT GAUNTLET + `agent.mdc`
   + `testing.mdc` + eval-pass Evidence + soft PROOF-EVALS rows.
3. Keep “do not invent mutation theater.”

## V10.1.3 — Martin gauntlet + gate audit

1. PRODUCT GAUNTLET block in User Rules (Martin: trust constraints, not
   reading agent code). Docs: `docs/AGENTIC-GAUNTLET.md`.
2. Ask gates for `gh release create` / `docker|podman push` and
   `find … -delete` / `rsync --delete`.
3. Heavy check matrix: `hooks/_audit_gate_matrix.py` in TOOLCHAIN.
4. testing.mdc + agent.mdc Done path cite house gauntlet.

## V10.1.2 — P* push / tree wipe

Plain `git push` and recursive `rm -rf` → ask; soft-without-gate does not
demote named MUST-NEVER/ASK classes.

## V10.1 patches (review of V10 vs counterexamples)

1. Zero-comment gate names **preToolUse** (deny before land) + **beforeShellExecution**;
   afterFileEdit is reactive only.
2. Platform fail-open documented; gate bodies wrap exceptions → deny; TOOLCHAIN mirrors checks.
3. PRECEDENCE: Evidence is #3 (after MUST-NEVER + confirmed intent).
4. ASK ONCE destructive = syntactic classes again; surgical delete ≠ tree wipe;
   restored “no trading correctness for brevity.”
5. Invented APIs: verify or say unsure; unverified is default until a tool call confirms.

Kept from V10: PRIME OBEDIENCE contract goal; context-injection non-confirmation line.

## Install

```bash
FORCE_SKILLS=1 bash install.sh
python3 hooks/_selftest.py && python3 hooks/_proof_evals.py
```

Paste / inject `user-rules/USER-RULES.paste.txt` as User Rules.
