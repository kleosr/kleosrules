---
name: codebase-memory
description: >-
  Uses the codebase knowledge graph MCP for architecture, callers, impact, and
  structural search. Use when exploring unfamiliar code, tracing call chains,
  finding definitions/dependencies, dead code, cross-service edges, or when
  Grep would dump the tree. Prefer over blind workspace-wide search.
---

# Codebase memory

Graph first for structure. Read files for truth. No speculation.

## Server

MCP server id: `user-codebase-memory-mcp`.

Always `GetMcpTools` for that server (or the specific tool) before
`CallMcpTool`. Discover schemas; do not invent parameters.

## When to use

- Who calls X / what X calls / impact of a change
- Where a symbol lives; architecture seams; cross-service edges
- Dead code, fan-in/fan-out, refactor blast radius
- Unfamiliar repo navigation before editing

Use Grep/Glob/Read for exact text, known paths, or tiny local edits.
Use this skill when structure or relationships matter.

## Decision matrix

| Question | Tool |
|----------|------|
| Indexed? | `list_projects` / `index_status` |
| Who calls X? | `trace_path` inbound |
| What does X call? | `trace_path` outbound |
| Full neighborhood | `trace_path` both |
| Find by name | `search_graph` name_pattern |
| Read symbol body | `search_graph` then `get_code_snippet` |
| Architecture | `get_architecture` |
| Diff impact | `detect_changes` |
| Cross-service edges | `query_graph` (Cypher) |
| Text / string | `search_code` or Grep |

## Workflows

### Explore

1. `list_projects` — confirm index; `index_repository` only if missing/stale
   or the user asked.
2. `get_architecture` or `search_graph` for the target.
3. `get_code_snippet` / Read the real file before any claim or edit.

### Trace

1. `search_graph` → exact name / qualified_name.
2. `trace_path` with depth as needed (`both` when cross-service may matter).
3. `detect_changes` when local diff impact matters.

## Hard stops

- Do not add MCP servers (`agent.mdc`). Use this one if already configured.
- Do not `delete_project` or re-index the world unless asked.
- No secrets in Cypher, traces, or chat.
- Graph hits are leads, not proof of runtime behavior — verify with Read and
  TOOLCHAIN when claiming Done.
- Institutional conclusions → distill to `AGENTS.md` / skills / Obsidian vault
  (skill `obsidian-memory`); do not leave alpha only in the graph session.
- This skill is the **code/AST** graph. Durable intent / decisions / sessions
  live in Obsidian (`user-obsidian`) — not here.

## Gotchas

- `trace_path` needs exact names — discover with `search_graph` first.
- Relationship filters on `search_graph` are degree filters; real edge lists
  often need `query_graph`.
- `query_graph` is row-capped — paginate / tighten the match.
- Check `has_more` / `offset` when results truncate.
- `outbound`-only misses many cross-service callers — prefer `both` when unsure.

## Fleet

Pairs with `agents-map` (write the map), `workspace-scope` (lock boundary),
`system-wiring` (end-to-end hops), `formulary` (kickoffs must point here for
structural navigation instead of dumping the tree into the prompt).
