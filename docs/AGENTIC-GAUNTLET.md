# Agentic gauntlet — Martin lens on Master Mind V11

Robert C. Martin (Uncle Bob): he does not read agent-written code. He
surrounds agents with extreme constraints — unit tests, Gherkin, QA,
quality metrics, mutation testing, coverage, and more — and trusts
output that survives that gauntlet **as far as that finite gauntlet reaches**.

V11: green product checks are not absolute semantic correctness
([`MECHANICAL-INCOMPLETENESS.md`](MECHANICAL-INCOMPLETENESS.md),
[`evals/MECHANICAL-INCOMPLETENESS-PSTAR.md`](evals/MECHANICAL-INCOMPLETENESS-PSTAR.md)).

Sources (discipline family): Clean Coders *Agentic Discipline*; his
mutation-testing notes; public write-ups of the Specifier → Gherkin →
Coder → Architect (mutation) pipeline.

## Two gauntlets (do not conflate)

| Gauntlet | What it protects | Where it lives in this pack |
|----------|------------------|-----------------------------|
| **Harness / obedience** | Agent cannot silently force-push, wipe trees, land prose comments, skip publish confirmation | User Rules + `hooks/` + proof evals |
| **Product / quality** | Human need not read the code; behavior is constrained by tests and metrics | Per-app `TOOLCHAIN.md`, CI, Gherkin, mutation, coverage — **not** invented by this pack |

Master Mind V10.1.3 is strong on the harness gauntlet. Martin’s
confidence model is the product gauntlet. Both are required for
“ship without reading.”

## Alignment

- **Evidence before Done** (MUST-NEVER + LOOP + `agent.mdc` QUALITY) matches
  “confidence from constraints, not vibes.”
- **eval-pass** is a process second opinion — not mutation testing.
- **testing.mdc** is public-surface / regression-oriented — not a full
  Gherkin or mutation pipeline.
- **Hooks** do not run app unit tests; they gate tool use.

## Honest gaps (heavy check, 2026-07-24)

Named MUST-NEVER / ASK classes without a complete live gate remain soft
until closed (fix hook, do not demote roof):

| Class | Status |
|-------|--------|
| Force-push / plain `git push` / `gh release` / `docker push` | allow (lab; `shell.deny` empty — V16.0.17+) |
| Tree `rm -rf` | ask (live) |
| `find … -delete` / `rsync --delete` | ask (live) |
| Shell tee/heredoc into CODE_EXT | deny (opaque_write; V16.0.18) |
| Native Delete treeish / extensionless | deny (live; `delete.json`) |
| `git reset --hard` / `git clean -fdx` | soft J when ungated (lab shell deny empty) |
| Invented APIs / skip verification | soft (policy + spot-check) |
| Product mutation / Gherkin / coverage bar | **per-repo only** — pack does not supply |

## Practice

1. Put the product gauntlet in each app’s `TOOLCHAIN.md` (or CI).
2. Agent: run that gauntlet before Done; cite the tool result.
3. Human: spot-check specs/Gherkin and Architect outputs — not every line
   of agent code (Martin strategy).
4. When a constraint is missing and confidence requires it: say so; do
   not pretend Soft Looked Good equals mutation green.

## P* (Martin stack coherence — closed in V10.1.4)

See [`evals/MARTIN-GAUNTLET-PSTAR.md`](evals/MARTIN-GAUNTLET-PSTAR.md).
Done tier-3 silent ship without a house gauntlet is no longer allowed for
non-trivial feature/ship asks — ASK ONCE (accept-risk or wire verify).
