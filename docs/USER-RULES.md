# USER RULES — how to load Master Mind V15.4

## Preferred (Settings User Rules)

1. Open `user-rules/USER-RULES.paste.txt`
2. Cursor → Settings → Rules → User Rules
3. Replace the **entire** box with that file
4. Keep `user-rules/option-c-core.mdc` at `alwaysApply: false`
5. New agent chat

## Disk fallback (when Settings MCP inject is unavailable)

1. `user-rules/option-c-core.mdc` + `~/.cursor/rules/option-c-core.mdc` at `alwaysApply: true`
2. Clear or replace any stale Settings User Rules box (avoid V11 + V15.4 double load)
3. New agent chat

SINGLE SOURCE: Settings paste **or** option-c alwaysApply — not both with different bodies.

V15.4: MUST-NEVER/M (mechanical) vs MUST-NEVER/J (judgment); see
`docs/evals/PERFORMATIVE-TRILEMMA-PSTAR.md`.

Layout is topological (U-curve): primacy roof → mid dictionary → recency
execution gates; hooks re-anchor at write time (`docs/TOPOLOGICAL-PROMPT.md`).

Then: `FORCE_SKILLS=1 bash install.sh`
