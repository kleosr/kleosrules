# Toolchain: Bash & jq Validation

No Rust. No Cargo. No Python. Mechanical checks are Bash.

## Prerequisites

- `bash` (v4+)
- `jq` (JSON parsing in hooks)
- Cursor IDE (runs the hooks)
- Obsidian MCP configured locally

## Syntax check

```bash
chmod +x hooks/*.sh

bash -n hooks/session_start.sh
bash -n hooks/before_submit_prompt.sh
bash -n hooks/stop_gate.sh
```

## Smoke test

```bash
echo '{"prompt": "test code", "hook_event_name": "beforeSubmitPrompt"}' \
  | bash hooks/before_submit_prompt.sh
```

Expect JSON with `additional_context` (and, on stop fixtures, `followup_message` when INTENT is missing).

## Size roofs

Keep each hook under 80 LOC. Fail closed: bad parse or failed exec should block or return deny JSON. Review catches the rest.
