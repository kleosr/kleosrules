AGENTS.md map of this pack

LAW vs MAP
- Law: user-rules paste + project-rules + hooks scripts
- This file: navigation only

Overview
kleosr V2 Bash hooks plus Obsidian MCP memory. Brain equals vault.
Muscle equals hooks scripts. No Rust. No Python pack tooling.

Where to look
- Paste: user-rules/USER-RULES.paste.txt
- Hooks: hooks/session_start.sh before_submit_prompt.sh stop_gate.sh
- Architecture: docs/ARCHITECTURE.md
- Curator: docs/CURATOR.md
- Verify: docs/TOOLCHAIN.md
- HANDOFF: HANDOFF.md

Done
chmod +x hooks scripts then bash -n on the three hook scripts

Hard stops
- Never reintroduce Rust kleos-gate or pack Python
- Never use updated_input in hooks only additional_context
- Secrets never in paste hooks or chat
