# Release — kleosr (current: V16.0.22)

## V16.0.22 — pack_native React + rules honesty (Rust ≠ Python)
1. `pack_native` allows PascalCase `.tsx`/`.jsx`/`.vue`/`.svelte`; snake function
   gate off on those + CSS/HTML.
2. Paste / option-c / vernacular.mdc: Rust-only pack tooling; Python/shell
   file-write = anti-bypass; `.sh` in lean ≠ pack bash; no fingerprint dodge.
3. Pack `16.0.22`. Residual: re-paste Settings User Rules; `python script.py`
   internal writes still argv-opaque.

## V16.0.21 — Kill Python Write bypass after recall/fingerprint

1. Recall deny no longer fingerprints — retry keeps `vault_read` route (no fingerprint
   freeze that pushed agents to Python).
2. Shell denies `python`/`python3` file writes (`write_text` / `open(w|a|x)` / Path.write).
3. Deny messages + stop LOOP_MSG: Cursor Write/StrReplace only after recall; never
   Python/shell bypass.
4. Pack `16.0.21`. Residual: `python script.py` that writes inside the script body
   still opaque to argv scan; re-paste optional.

## V16.0.20 — Recall M-on + event-loop weekly nudge

1. `context.json` `recall_gate_enabled: true` — CODE_EXT write denies without ledger
   `obsidian_recall` (exempt: docs/skills/user-rules/project-rules/hooks/policy).
2. sessionStart injects EVENT LOOP nudge when `wiki/audits/*weekly*` missing or >7d.
3. Honesty table + paste/option-c/skill/curator: recall ON (M Total when enabled).
4. Pack `16.0.20`. Residual: re-paste Settings User Rules; J followup/stub meter (Rice);
   OS cron still optional beyond sessionStart nudge.

## V16.0.19 — Retire Life OS brand

1. Delete vault `instructions/LIFE-OS.md` — habits stay in PROCESSING + AGENT-MEMORY.
2. User Rules / option-c: strip Life OS pointers; keep GRAPH DISCIPLINE + skill Read.
3. Pack `16.0.19`. Residual: re-paste Settings User Rules; new chat.
4. Completeness C′ closes: ask-scope `enabled: true` (P*-17 restore); `delete.json`
   `deny_extensionless_basename` (P*-18); honesty table Force-push → lab ACT /
   Native Delete treeish M; opaque residual text → deny; recall-overclaim scrub.

## V16.0.18 — Deny shell file writes (force Write/StrReplace)

1. `shell.json` `opaque_write_deny_message` — tee/heredoc/redirect into CODE_EXT
   or CSS hard-deny; message routes to Write/StrReplace.
2. `lean.json` code_extensions + `.css`/`.scss`/`.html` (wood-rail CSS cannot
   bypass via Shell).
3. `git apply` / `patch` still allow. Only `rm` still asks.
4. Lab fluid kept: recall off (superseded V16.0.20), shell deny empty, lean ON.

## V16.0.17 — Lab fluid (only rm asks)

1. `context.json` `recall_gate_enabled: false` — no write deny for missing vault recall
  (fixes Metanoia fingerprint freeze from recall races).
2. `lean.json` `enabled_default: true` — lean meter stays ON (size roofs); `KLEOS_LEAN=0` to disable.
3. `shell.json` `deny: []`; `ask` = recursive rm + find -delete / rsync --delete only.
4. Still deny: prose comments, secrets, vernacular, Delete tree, subagent force brief.
5. User Rules V16.0.17 + option-c sync. Residual: re-paste Settings.

## V16.0.16 — wandermist graph gut check

1. Ingest: `wiki/sources/Wandermist-Everyone-Wrong-Graph-Engineering` (X Article FULL).
2. User Rules / option-c: GRAPH DISCIPLINE in LOOP (≥2 of 4 before fan-out);
   DOC MAP pointer; LIFE-OS parallel + gut check.
3. Pack `16.0.16`. Residual: re-paste Settings if paste is SSOT.

## V16.0.15 — Life OS + skill Read obedience (User Rules)

1. User Rules / option-c: SKILL ROUTING requires Read SKILL.md on match;
   Life OS (`instructions/LIFE-OS.md`) in INTENT + DOC MAP + LOOP;
   routes for ship-loop / session-handoff / eval-pass / harness-retro /
   bug-hunt / agents-map / formulary / `/loop`.
2. Pack `16.0.15`; disk mirror synced to `~/.cursor/rules/option-c-core.mdc`.
3. Residual: human must re-paste `USER-RULES.paste.txt` into Cursor Settings
   if Settings paste is the live constitution (SINGLE SOURCE — do not run
   both paste and alwaysApply option-c with divergent bodies).

## V16.0.14 — Lab auto-allow asks (no human Approve)

1. Policy: `shell.json` `ask: []`; `opaque_write_ask_message: ""`;
   `ask-scope.json` `enabled: false`. MCP danger already `a^`.
2. Hard deny unchanged: force-push, recursive rm, secrets, prose, lean, vernacular.
3. User Rules / option-c: remote publish ACT (lab); sovereign override logged.
4. Meters: opaque/push bench+tests expect `allow`; ask-scope enable still proven via copy.

## V16.0.13 — P*-16 Staircase Composition kill

1. P*: `docs/evals/STAIRCASE-COMPOSITION-PSTAR.md`.
2. Kill: absolute post-state `file_loc_max: 700` (+ `KLEOS_LEAN_FILE_LOC_MAX`)
   on Write and projected StrReplace; honesty table split increment vs absolute;
   doctrine: per-event bounds do not compose to state bounds.
3. Meters: 5 unit tests in `engine/lean.rs` (was 0), incl. staircase regression.
4. Measured: `sup(file LOC | all events green)` ∞ → 700.
5. Residual: delete.rs sequential-singles sibling (same quantifier class);
   extension allowlist; vernacular non-hermetic test.

## V16.0.12 — P*-17 Dual-Write Lean + ask-scope re-enable

1. P*: `docs/evals/DUAL-WRITE-LEAN-PSTAR.md`.
2. Kill: Shell heredoc/redirect embedded CODE_EXT → prose + vernacular + lean
   (parity with Write); opaque CODE_EXT without embedded body → ask
   (`opaque_write_ask_message` restored); `ask-scope.json` `enabled: true`.
3. Meters: `shell_heredoc_oversize_denies_lean`, `shell_tee_code_asks_opaque`,
   `ask_scope_enabled_asks_drive_by`; bench cases.
4. Residual: tee/sed without argv body still asks (confirm); template prose FN;
   lean size ≠ ∀ quality (P*-15).

## V16.0.11 — Fleet TOOLCHAIN fill + create-on-ship

1. Evidence-only `TOOLCHAIN.md` written across Fleet catalog gaps (Victoria,
   mono_ai-portfolio, mx-live-feeds, Fitness, contrib-fill, CLP, Notes,
   Research, Skills) + AGENTS Done pointers where needed.
2. Policy: missing TOOLCHAIN on scanned fleet root + real scripts/CI → create
   same changeset then run (`agent.mdc`, User Rules paste, `agents-map`).
3. Existing crm / Terremoto* / mvpMedico / rules TOOLCHAIN drift-checked OK.
4. Never invent tests; never ask accept-no-gauntlet-risk.

## V16.0.10 — Five-layer residual squeeze (plan execute)

1. `docs/LAYER-STACK.md`: full 6 X sources; 0xJeyx 6-step org graph;
   when-not-to-graph; fourth complementary graph (org/workflow).
2. `harness-retro` classifies by five-layer unit; Rust install/verify only.
3. `ship-loop` / `eval-pass`: model stop ≠ Done; eval-pass = reviewer node.
4. Vault: 0xJeyx source + catalog; Agential-Control stub; link hygiene.
5. Evidence: cargo test + kleos-gate verify/install required on land.
6. Master Mind paste / option-c / agent banners → **V16.0.10** (was V15.7 display lag).

## V16.0.9 — Write-back ≠ wipe (max memory wording)

1. Roof strings: “Obsidian flush” → **write-back / persist INTO vault**.
2. Compaction message: saves memory before chat dies; does **not** clear vault.
3. Skill + companion + LAYER-STACK wording aligned. Max memory requires write-back.

## V16.0.8 — Layer stack + Graph Engineering squeeze

1. Doctrine: `docs/LAYER-STACK.md` — prompt→context→harness→loop→graph units;
   maps onto User Rules / wiki curation / kleos-gate / ship-loop / Obsidian+AST.
2. AGENTIAL-CONTROL links layer stack (orthogonal to persuasion/force/memory).
3. Vault: Five-Layer + Graph Eng + Process-Not-Patches concepts; PROCESSING
   graph-maintenance classes; dual/tri graphs; sources from akshay/Sprytix/undefinedKi.
4. Skill + companion + agent DOC MAP pointers. No Neo4j/GraphRAG product bolt-on.
5. Residual: X fetch may fail; paste User Rules if Settings lag; optional ASK
   for GraphRAG/Neo4j only if product need appears.

## V16.0.7 — Auto house gauntlet (no human accept-risk)

1. Stop followup: ACT NOW run TOOLCHAIN/verify yourself — never ask
   accept-no-gauntlet-risk.
2. PRODUCT GAUNTLET + agent.mdc + testing.mdc + option-c/paste aligned.
3. Broader verify-class regex (`kleos-gate verify|bench|gate-diff`,
   `npm run lint|check|typecheck|build`, `check:domain`).
4. Residual: agent must still execute the Shell verify; followup is force nudge.

## V16.0.6 — Unblock installs/MCP; hard-stop true destruction

1. Package install / npx / MCP danger-ask / ask-scope / opaque shell write → allow (ACT).
2. Recursive `rm -rf`, find-delete, rsync --delete → deny (not ask).
3. Remote publish (`git push`, release/image push) still asks.
4. User Rules + option-c → **V15.7** (installs ACT). Session roof drops ASK installs.
5. Residual: Cursor product UI may still confirm some tools; secrets + prose comments still deny.

## V16.0.5 — Max memory fleet refeed

1. Ingest Cyril/Degen/Iolld threads; concepts for Kimi agent, skills+qmd, max refeed.
2. Seed `wiki/projects/*` for every Documents coding repo + notes/research/skills-docs.
3. `instructions/MAX-MEMORY.md` + skill/companion continuous inject loop.
4. Catalog: `wiki/catalogs/Fleet.md`. Residual: optional ASK for kepano skills / qmd MCP.

## V16.0.4 — Complete second brain (ingest/query/lint + write-back)

1. Harvest Dezo + Obsidian/LLM-Wiki status threads into vault catalog.
2. Ops triad + `wiki/log.md`, `raw/processed/`, concepts/entities/sources/catalogs.
3. Skill + companion: context ≠ memory; model = reasoning; vault = SSOT; write-back mandatory.
4. Cite: [0xDezo](https://x.com/0xDezo/status/2079595162955571339),
   [Karpathy gist](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f).

## V16.0.3 — Karpathy LLM Wiki vault layout

1. Vault `/home/kleosr/rootsidian/kleosr` → `raw/` · `wiki/` · `instructions/`.
2. Special files: `wiki/hot.md`, `wiki/index.md`, `instructions/PROCESSING.md`,
   root `AGENT.md`. Concepts under `wiki/concepts/`; projects under `wiki/projects/`.
3. Skill + companion + kleos-gate recall/flush strings point at wiki paths;
   never edit `raw/`; query wiki only.
4. Cite: [0xkkai / Karpathy LLM Wiki](https://x.com/0xkkai/status/2081005037992464894).

## V16.0.2 — Obsidian amnesia roofs (auto recall/flush)

1. kleos-gate `sessionStart` injects mandatory Obsidian recall context.
2. `stop` followup if tool work ran with no vault write logged; `preCompact`
   flush warning; mid-session duty nudge every 12 tools.
3. Companion + skill: every-chat loop + `journals/YYYY-MM-DD.md` mid-capture.
4. Residual: Obsidian app must run; agent must obey flush followup (J until write).

## V16.0.1 — Obsidian memory graph wired

1. Skill `obsidian-memory` + always-on companion `obsidian-memory.mdc`.
2. MCP `user-obsidian` = durable knowledge graph (wikilinks); AST graph stays
   `codebase-memory`. Session start/end + dual-write with `HANDOFF.md`.
3. USER-RULES / option-c / agent SESSION / AGENTIAL-CONTROL / EPISTEMIC-PERSIST
   updated. Vault seed under `/home/kleosr/rootsidian/kleosr`.
4. Residual: Obsidian app must be running; vault ≠ TOOLCHAIN proof.

## V16.0.0 — Shell zero in pack (Rust fleet CLI)

1. Port pack shell to `kleos-gate` fleet subcommands: `install`, `install-hooks`,
   `sync`, `sync-hooks`, `verify`, `bench`, `discover`, `install-pre-commit`.
2. Delete `install.sh`, `hooks/install-user-hooks.sh`, `scripts/*.sh`,
   `lib/discover-repos.sh`; drop empty `scripts/` + `lib/` maps.
3. TOOLCHAIN / CI / package.json / AGENTS / README → Rust CLI only.
4. Crate version `16.0.0`. Vernacular prefixes drop `scripts/` / `lib/`.
5. Residual: git pre-commit wrapper body is still bash (git hook substrate);
   Cursor data surfaces remain md/mdc/json (not executables).
6. Brand: pack identity **kleosr** (`package.json` / README / maps); binary stays
   `kleos-gate`. Pack vernacular SSOT: `project-rules/vernacular.mdc`.

## V15.6.0 — P*-15 Lean Size ≠ Semantic Quality kill

1. P*: `docs/evals/LEAN-SIZE-QUALITY-PSTAR.md`.
2. Kill: claim scope — lean meter = finite size M roofs; semantic quality /
   clean / YAGNI = J soft ladder. Honesty table row + NATIVE LEAN wording.
3. Touch: USER-RULES + option-c-core, ponytail/lean-code/agent/native-lean
   companions, ponytail skill, doctrine (AGENTIAL-CONTROL, COGNITIVE-COLLAPSE,
   DEFECT-COMPENSATION), breakthrough chains, README/AGENTS/rules.
4. No Rust / policy schema change. kleos-gate crate remains 15.4.0.
5. Residual: legal mediocre diffs under size caps; extra meters still gameable.

## V15.5.0 — P*-14 Soft-Force Schism kill

1. P*: `docs/evals/SOFT-FORCE-SCHISM-PSTAR.md`.
2. Kill: skill Self-target pause (`skills/unconditional-counterexample/SKILL.md`);
   soft = J-authority when routed (never waive M); README/contract slogan scoped.
3. Touch: USER-RULES + option-c-core, agent.mdc, README, breakthrough chains.
4. No Rust / policy schema change. kleos-gate crate remains 15.4.0.
5. Residual: chat emission ungated by design; Self-target pause is J not force.

## V15.4.0 — P*-13 Performative Trilemma kill (language)

1. P*: `docs/evals/PERFORMATIVE-TRILEMMA-PSTAR.md`.
2. Kill: MUST-NEVER/M vs MUST-NEVER/J; ENFORCEMENT HONESTY TABLE; scoped
   distrust; verified intent evaluated against gates (not defined by them);
   J sovereign override (logged); no agent-initiated weakening.
3. Touch: `USER-RULES.paste.txt`, `option-c-core.mdc`, `project-rules/agent.mdc`,
   `docs/AGENTIAL-CONTROL.md`, breakthrough chains, AGENTS/TOOLCHAIN versions.
4. Zero Rust / policy schema change — language now matches `main.rs` force surface.
5. Residual: mechanism A2 (template prose FN, shell argv smuggle, lexical ask-scope).

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
cargo build --release --manifest-path hooks/kleos-gate/Cargo.toml
mkdir -p hooks/bin && cp -f hooks/kleos-gate/target/release/kleos-gate hooks/bin/kleos-gate
FORCE_SKILLS=1 hooks/bin/kleos-gate install
(cd hooks/kleos-gate && cargo test) && hooks/bin/kleos-gate bench
```

Paste / inject `user-rules/USER-RULES.paste.txt` as User Rules.
