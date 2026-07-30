ARCHITECTURE — kleosr V2

Five layers
1. Prompt — user message; model starts empty
2. Context — HANDOFF tail + before_submit_prompt classify + Obsidian hot/index
3. Harness — Cursor + Bash hooks + tools
4. Loop — Done-when + stop_gate followups
5. Graph — Obsidian vault Sessions/hot (durable)

Amnesia preventiva
Cursor window dies. Obsidian vault survives.
Hooks force read wiki/hot.md + wiki/index.md and write Session + hot.

Injection vs Declaration
Hooks emit additional_context only. Never updated_input.
Agent declares INTENT: and Done-when: in chat before write tools.

Hook map
- sessionStart -> session_start.sh (HANDOFF tail 15 + amnesia duty)
- beforeSubmitPrompt -> before_submit_prompt.sh (ROUTE_CLASSIFY + DEBERES)
- stop -> stop_gate.sh (require INTENT + Done-when; order Obsidian Session)

Session path
wiki/projects/project/Sessions/YYYY-MM-DD-topic

Residual
Former Rust M roofs (comments, lean LOC, vernacular machine, secrets regex, recall deny) are not reimplemented in this Bash pass.
