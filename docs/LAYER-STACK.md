# Layer stack — prompt → context → harness → loop → graph

Terms stack; they do not replace each other. Each wraps the one below.
Debug by unit of work: find which layer’s unit broke, then fix that layer
([akshay units](https://x.com/akshay_pachaar/status/2081356379026280677)).

## Units

| Layer | Is | Unit of work | kleosr surface |
|-------|-----|--------------|----------------|
| **Prompt** | Message — model starts empty each call | One input | User Rules U-curve Blocks 1–3; companions; vault `AGENT.md` / `instructions/*`; skill prompts |
| **Context** | Memory / curation — finite window | What stays in the window | Force: playbook + capped hot + **ask classify/pointers** + meter + **recall deny on CODE_EXT** when `recall_gate_enabled` (`hooks/policy/context.json`; default **true**) + weekly-lint EVENT LOOP nudge. Agent: targeted MCP + vault duty before CODE. Ledger: COMPLETE write-back INTO vault; INTENT restatement + Session LAYER CHECK (see [`CURSOR-CURATOR.md`](CURSOR-CURATOR.md)). Playbook never rewrites the user prompt. |
| **Harness** | Machine — gather → tools → verify | One pass | Cursor + MCP + `kleos-gate` (deny/ask roofs) + TOOLCHAIN verify; judge = green checks |
| **Loop** | Run — goal, brakes, completion | Whole run | stop ACT-NOW verify followup; ship-loop / session-handoff; max retries; goal in Session note |
| **Graph** | Coordination — nodes, edges, shared state | Whole job | **Knowledge:** Obsidian wikilinks. **AST:** codebase-memory. **Work lineage:** git / HANDOFF / Sessions. **Org/workflow:** skills + subagents + eval-pass as reviewer (not Neo4j) |

Nesting: prompt + context sit inside harness gather. Harness = one pass.
Loop decides whether to run the pass again. Graph decides which loops run
([akshay deep dive](https://x.com/akshay_pachaar/status/2081089131808243999)).

## Squeeze map (max density, no theater)

1. **Prompt** — keep roofs short (User Rules). Ops detail in skills/companions/vault instructions — do not paste GraphRAG essays into always-on rules.
2. **Context** — playbook + hot + ask classify/pointers; INTENT restatement (2B); agent curates drill-down; COMPLETE capture heuristics + stop followups; never re-load all of `raw/`; AST graph for code. See `CURSOR-CURATOR.md`. No Neo4j product.
3. **Harness** — mechanical roofs stay M; auto gauntlet runs TOOLCHAIN without human accept-risk.
4. **Loop** — Session goal + Done evidence + Open; stop followup forces another pass when unverified. Model stop ≠ Done.
5. **Graph** — durable facts as `[[wikilinks]]`; ingest classifies new/dup/contradiction/update/uncertain; org graph via skills — not a hand-rolled graph runtime.

## Graph Engineering vs RAG (for this pack)

Regular RAG = chunk search. Graph Eng = explicit relationships
([Sprytix](https://x.com/Sprytixl/status/2081393802359505153);
[Sprytix companion](https://x.com/Sprytixl/status/2078778799064584535);
[Microsoft GraphRAG](https://github.com/microsoft/graphrag)).

kleosr ships a **personal knowledge graph** as Markdown + wikilinks (LLM Wiki).
Do not bolt on Neo4j/GraphRAG product unless asked.

Process rule ([undefinedKi](https://x.com/undefinedKi/status/2080992300893675775)):
**fix the rulebook / judge / queue — not hand-patch agent output.** Skill
`harness-retro` is that move for pack law.

## Complementary graphs (do not collapse)

| Graph | Answers |
|-------|---------|
| Knowledge (Obsidian) | What entities/decisions exist; how they connect |
| AST (codebase-memory) | Who calls what in code |
| Work lineage (git / Sessions / HANDOFF) | What changed; which experiment/session parent |
| Org / workflow (skills + subagents) | Who runs when; reviewer with teeth; shared artifact contracts |

## Agent-team org graph (0xJeyx — 6 steps)

When several loops must coordinate ([0xJeyx](https://x.com/0xJeyx/status/2080282086577979709)):

1. **Nodes only if real specialties** — planner / implementer / reviewer / judge, not theater roles.
2. **Draw edges before code** — what runs after what; fan-out/fan-in explicit.
3. **Explicit shared state** — who writes wiki / HANDOFF / artifacts; disk over chat.
4. **Reviewer node with teeth** — `eval-pass` separate from implementer; TOOLCHAIN is the mechanical judge.
5. **Isolate failure** — named node / layer; fix that layer; do not restart the whole job blind.
6. **Prefer existing orchestration** — Cursor + kleos-gate + skills + vault. No pack LangGraph/CrewAI/bash queue theater.

**When not to graph:** one finish line, sequential steps, same tools → stay in one loop.

## Debug checklist

| Symptom | Likely layer |
|---------|----------------|
| Bad one-shot answer, missing role/format | Prompt |
| Forgot prior decision; window full of junk | Context |
| Tools not run / no verify / gate thrash | Harness |
| Stopped mid-task; “done” without goal met | Loop |
| Parallel agents clash; no shared state; orphan notes | Graph |

## Sources

| Author | URL | Role |
|--------|-----|------|
| @akshay_pachaar | https://x.com/akshay_pachaar/status/2081356379026280677 | Units |
| @akshay_pachaar | https://x.com/akshay_pachaar/status/2081089131808243999 | Nesting / deep dive |
| @Sprytixl | https://x.com/Sprytixl/status/2081393802359505153 | KG vs RAG |
| @Sprytixl | https://x.com/Sprytixl/status/2078778799064584535 | Graph Eng companion |
| @undefinedKi | https://x.com/undefinedKi/status/2080992300893675775 | Process graph (judge/rulebook) |
| @0xJeyx | https://x.com/0xJeyx/status/2080282086577979709 | Agent-team org graph (6 steps) |

## Related

- Control stack (orthogonal): [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md)
- Vault ops: skill `obsidian-memory` · vault `instructions/MAX-MEMORY.md`
- Martin gauntlet: [`AGENTIC-GAUNTLET.md`](AGENTIC-GAUNTLET.md)
- Topological prompt: [`TOPOLOGICAL-PROMPT.md`](TOPOLOGICAL-PROMPT.md)
- Retro: skill `harness-retro` (classify by five-layer unit)
