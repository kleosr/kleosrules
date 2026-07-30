hooks AGENTS nested map. Parent: ../AGENTS.md
Scope: Bash Cursor hooks for INTENT inject, stop audit, HANDOFF tail.
Scripts: sessionStart -> session_start.sh ; beforeSubmitPrompt -> before_submit_prompt.sh ; stop -> stop_gate.sh
Registries: hooks.json , hooks.project.json
Done: chmod +x then bash -n on hook scripts
Hard stops: never updated_input ; never Rust or Python gate ; each script max 80 lines
