# kleosrules V2: Bash + Obsidian Context Engineering

Cursor harness pack for the 5 Layers framework. Governance is Bash; memory is Obsidian MCP. No Rust. No Python.

Requires Obsidian (app running) plus an Obsidian MCP server in Cursor. Without that, durable memory does not work.

Platform: Linux or WSL. Native Windows is experimental (I am testing it). If you want a PowerShell port, feel free.

How it fits Cursor: Cursor is where you build. Chats are focused and finite by design. This pack pairs that with Obsidian so decisions and sessions persist across chats. Hooks nudge a quick vault read at start and a write-back when you finish, so context carries forward without cluttering the window.

## Setup (no Cargo)

1. Hooks: use the `hooks/` directory in this pack (or copy it into your project).
2. Permissions: `chmod +x hooks/*.sh`
3. Wire: register `session_start.sh`, `before_submit_prompt.sh`, and `stop_gate.sh` in `.cursor/hooks.json` (this pack ships `hooks/hooks.json`).
4. Rules: paste `user-rules/USER-RULES.paste.txt` into Cursor → Settings → Rules → User Rules.
5. MCP: point Obsidian MCP (`user-obsidian`) at your vault so Layer 5 stays durable. Obsidian must be installed and open.

Needs `bash` and `jq` on PATH.

## The loop (injection vs declaration)

1. Prompt: you send a message.
2. Inject (Layer 2): `before_submit_prompt.sh` adds duties with `additional_context`. It does not mutate the user prompt.
3. Declare (Layer 1): the agent writes `INTENT:` and `Done-when:` in chat before tools run.
4. Audit (Layer 3/4): `stop_gate.sh` checks `Done-when`. If unmet, it forces another pass. If met, it clears `/state` and asks for Obsidian write-back.

## Core structure

- `hooks/`: Bash scripts (fail-closed, max 80 LOC each)
- `state/`: ephemeral run files (`current_intent.md`), gitignored
- `docs/`: Architecture (5 Layers), Toolchain, Curator
- `HANDOFF.md`: strict transfer file; `session_start.sh` injects `tail -n 15`

## License

MIT.
