# USER RULES — how to load Master Mind V16.0.19

## Preferred (Settings User Rules)

1. Open `user-rules/USER-RULES.paste.txt`
2. Cursor → Settings → Rules → User Rules
3. Replace the **entire** box with that file
4. Keep `user-rules/option-c-core.mdc` at `alwaysApply: false` when paste is SSOT
5. New agent chat

## Disk fallback (when Settings MCP inject is unavailable)

1. `user-rules/option-c-core.mdc` + `~/.cursor/rules/option-c-core.mdc` at `alwaysApply: true`
2. Clear or replace any stale Settings User Rules box (avoid double load)
3. New agent chat

SINGLE SOURCE: Settings paste **or** option-c alwaysApply — not both with different bodies.

Lab fluid (V16.0.17–19): package installs / npx / MCP are ACT; remote publish and
force-push are ACT (`shell.deny` empty); recursive `rm` / `find -delete` /
`rsync --delete` ASK; shell tee/heredoc into CODE_EXT **deny**; secrets + prose
comments still deny; recall gate off; lean ON; Native Delete treeish/extensionless
deny. House gauntlet is ACT NOW (agent runs TOOLCHAIN — never ask
accept-no-gauntlet-risk). Soft skills = J when routed (never waive M). See
`docs/evals/LEAN-SIZE-QUALITY-PSTAR.md`, `docs/evals/SOFT-FORCE-SCHISM-PSTAR.md`,
`docs/evals/DELETE-STAIRCASE-PSTAR.md`.

Layout is topological (U-curve): primacy roof → mid dictionary → recency
execution gates; hooks re-anchor at write time (`docs/TOPOLOGICAL-PROMPT.md`).

INTENT restatement + COMPLETE CAPTURE + Session LAYER CHECK named in paste
(gate + companions already force; paste is recovery SSOT). `docs/CURSOR-CURATOR.md`.

Then: `hooks/bin/kleos-gate install`
