# Context Curation & Memory Management

Layer 2 is mostly about what you throw away.

## Ephemeral state (`/state/`)

Local context is volatile. `/state/` holds atomic files for the current run:

- `current_intent.md`: overwritten on every prompt
- `pending_context.md`: short-lived injected duties

Clear `/state/` after a successful loop so old intent does not poison the next one.

## HANDOFF.md

Structured state at the repo root.

- **Format:** TAREA, ARCHIVOS, ESTADO, SIGUIENTE, INTENT
- **Injection:** `session_start.sh` takes only the last 15 lines (`tail -n 15`)
- **Update:** `stop_gate.sh` makes the agent refresh this file before stop

## Durable graph (Obsidian MCP)

The Cursor window is finite. Obsidian is the long-lived graph.

- **Read:** at session start (or when you need history), query `hot.md` or `index.md` via MCP
- **Write:** when `Done-when` is met, write `wiki/projects/<project>/Sessions/<YYYY-MM-DD>-<topic>` and refresh `hot.md`

Do not mix ephemeral scratch with durable memory. If it must outlive the chat, put it in Obsidian.
