# Grok 4.5 + Cursor harness notes

## What Grok is good at here

- Parallel tool use (read/search/edit/shell)
- Following a short constitution (agent.mdc) + dense maps (AGENTS.md)
- Implementing when Done is an explicit command list

## What wastes its strength

- Multi-thousand-token always-on style guides
- Step theaters ("PLAN:" with 15 fake steps every turn)
- Workspace-wide "cleanup" prompts
- Claude-era XML ritual and model-ID matrices irrelevant to this runtime

## Context budget (practical)

| Layer | Budget mindset |
|-------|----------------|
| User Rules | Thin identity + pointers |
| Project .mdc | ~100 lines law |
| AGENTS | ~80–120 lines map |
| Turn prompt | Goal + scope + Done |
| Skills | Load only when task matches |

## Tool-use expectations to encode

- Never speculate about unopened code
- Cite tool results before "done"
- Prefer **codebase-memory** (graph) for callers/architecture/impact; then Read
- Prefer Grep/Read for exact text and known paths — not as a substitute for
  structural tracing
- `GetMcpTools` on `user-codebase-memory-mcp` before calling its tools
- Shell verify from TOOLCHAIN only
- Prefer one model per conversation; handoff + new chat/subagent to switch
- Feature work → `ship-loop` + `session-handoff`; large diffs → `eval-pass`

## Anti-Claude-porting

Do not port: `budget_tokens`, Fable/Opus effort enums, Anthropic beta headers,
or "use Haiku for routing" tables into this fleet's User Rules or AGENTS.md.
If a third-party doc is Claude-only, translate intent (verify, short scope)
and drop the brand/API surface.
