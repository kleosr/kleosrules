---
name: formulary
description: >-
  Prompt and harness discipline for Grok 4.5 (xAI) coding agents in Cursor/Grok
  Build. Use when writing or reviewing system prompts, user prompts, agent
  kickoffs, tool-use instructions, or fixing agents that are verbose, skip
  tools, invent Done, ignore AGENTS/TOOLCHAIN, overengineer, or ship without
  evidence. Not for Claude/Anthropic model selection, Fable/Opus/Sonnet IDs,
  or non-Grok API param tables.
metadata:
  version: "2.1.0"
  stack: "Grok 4.5 + Cursor/xAI agent harness"
---

# Formulary — Grok 4.5 agent prompts (this fleet)

**Default model for this operator: Grok 4.5.** Claude-specific formulary
(Fable/Opus/Sonnet, Anthropic effort headers, snippet libraries from
platform.claude.com) is **out of scope** here. Do not load or recommend those
paths for work in this harness.

Goal: **correct tool-using agent behavior** — short context, real reads, real
verify — not creative essay prompts.

## When to use

- Writing/fixing a kickoff prompt, User Rule, skill body, or AGENTS map line
- Agent too verbose, skips TOOLCHAIN, fabricates progress, won't open files
- Hardening high-stakes agent turns (auth, money, crisis data, secrets)

## When NOT to use

- Picking Claude model IDs or Anthropic API params
- Pure coding with no prompt/harness text to write (just implement)
- Billing/account questions

## Grok 4.5 + this harness (truths)

| Prefer | Avoid |
|--------|--------|
| Goal + constraints + Done definition | Step-by-step micro-plans the model must recite |
| Point to AGENTS.md / TOOLCHAIN.md | Dumping half the codebase into the prompt |
| Point to **codebase-memory** for structure | Blind workspace-wide Grep / tree paste |
| "Graph → Read X then edit" | "You are an expert…" roleplay paragraphs |
| One boundary / one outcome | "Improve the whole workspace" |
| Evidence in the report | "Should be fine" / theater checklists |
| Skills on demand | Always-on novels |

Grok 4.5 is strong at agentic coding and parallel tools. Failures are usually
**harness/prompt**: wrong scope, no map, invented Done, or workspace thrash —
not missing Claude-style XML scaffolding.

## Workflow (when authoring a prompt)

1. **Name the job in one sentence** (outcome, not process).
2. **Attach law/map by pointer**, not paste: `.cursor/rules`, `AGENTS.md`,
   `TOOLCHAIN.md`. Full task in the first user turn when possible.
3. **Navigation**: if the job needs callers, architecture, impact, dead code,
   or unfamiliar structure, require skill **codebase-memory** (MCP
   `user-codebase-memory-mcp`: `search_graph` / `trace_path` /
   `get_architecture` then Read). Never rewrite a prompt to paste the tree.
4. **Scope**: smallest coherent app/package/crate/service/context; use
   **workspace-scope** and preserve existing topology.
5. **Done**: exact verify command class (TOOLCHAIN or "commands you actually run").
6. **Hard stops**: only non-discoverable landmines (PII, vault, deploy).
7. **Measure**: after the run, did the agent use the graph when structure
   mattered, read map, touch only scope, show verify evidence? If not, fix
   the prompt (usually shorter + sharper Done + codebase-memory pointer).

## Kickoff patterns (Grok-friendly)

### Default task

```text
Read AGENTS.md and TOOLCHAIN.md.
Follow codebase-memory for structure/callers (graph → Read); no tree dumps.
Task: <one sentence>.
Min diff. No scope expansion.
Done = TOOLCHAIN commands with evidence.
```

### Multi-boundary workspace

```text
Preserve this repository's topology.
SCOPE: <primary app/package/crate/service/context>.
Affected contracts: <none or exact interfaces and consumers>.
Read nested AGENTS if present. Trace cross-boundary with codebase-memory
(trace_path / search_graph) before editing. Verify primary boundary + contracts.
```

### Assessment only

```text
Assessment only. No edits. Open AGENTS/README.
Locate <X> via codebase-memory; cite graph hits and paths you Read.
Risks? Smallest plan for <Y>?
```

### Correctness-critical

```text
Trust boundaries matter. No secrets in chat/diffs. No invented facts —
label guesses. Structure claims from codebase-memory + Read. Done only with
tool evidence from this turn.
```

## Symptom → remedy

| Symptom | Remedy in the prompt |
|---------|----------------------|
| Narrates, little tool use | "Lead with outcome. Open files before claims. No filler." |
| Skips AGENTS/TOOLCHAIN | Explicit first line: read those paths |
| Greps forever / dumps trees | Require **codebase-memory** (graph → Read) |
| Speculates on unopened code | "search_graph/trace_path then Read; no claims without hits" |
| Touches whole workspace | SCOPE lock + workspace-scope skill |
| Invents `npm test` | "Only commands in TOOLCHAIN or package.json you opened" |
| Overengineers | "No abstraction before third repetition. Min diff." |
| Stops early without verify | "Red without evidence = not done" |
| Pastes secrets / .env | Hard stop line + agent.mdc SAFETY already |

Longer discipline rules (optional embed): `references/reasoning-discipline.md`.
Grok-oriented harness notes: `references/grok-harness.md`.

## Non-negotiables

1. **This skill is Grok 4.5 / xAI harness first.** Do not route the user to
   Claude model matrices.
2. **Pointers > paste.** Prefer repo maps and rules over giant system prompts.
3. **Done is binary with evidence.** No vibe-green.
4. **Topology is not the task.** Preserve it unless restructuring is explicitly
   required and approved (see agent.mdc + workspace-scope).
5. **Positive, short instructions** beat CRITICAL/MUST spam.

## Relation to the fleet

| Artifact | Role |
|----------|------|
| `agent.mdc` | Always-on law |
| `agents-map` | Build maps |
| `codebase-memory` | Graph navigate/trace; kickoffs must point here for structure |
| `workspace-scope` | Lock app/package/crate/service boundaries |
| `domain-architecture` | Model complex domains and bounded contexts |
| `prompts/cheatsheet.md` | Human copy-paste menu |
| This skill | How to write/fix prompts for Grok agents |
