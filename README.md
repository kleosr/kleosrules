kleosr V2

Cursor harness: Bash hooks + Obsidian MCP memory. No Rust. No pack Python.

Setup
1. Ensure jq is installed
2. chmod +x hooks/session_start.sh hooks/before_submit_prompt.sh hooks/stop_gate.sh
3. Point Cursor hooks at hooks/hooks.json (or copy scripts into ~/.cursor/hooks)
4. Paste user-rules/USER-RULES.paste.txt into Cursor User Rules
5. Enable MCP user-obsidian to vault /home/kleosr/rootsidian/kleosr

Loop
Prompt -> before_submit inject -> agent INTENT/Done-when -> tools -> stop_gate -> Obsidian Session + hot

Docs
- docs/ARCHITECTURE.md — five layers, amnesia, injection vs declaration
- docs/TOOLCHAIN.md — smoke verify
- HANDOFF.md — TAREA ARCHIVOS ESTADO SIGUIENTE INTENT

License: MIT
