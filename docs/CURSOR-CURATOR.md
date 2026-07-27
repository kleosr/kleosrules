# Cursor curator — what the agent must do

Force injects seed + ask hints. Agent still owns judgment. No Neo4j.
User prompt text is never rewritten — restate intent beside it (2B).

## Injected every session (kleos-gate)

1. Roof + Obsidian recall nudge  
2. **Playbook** (from `hooks/policy/context.json`) — INTENT + LAYER CHECK duties  
3. Capped `wiki/hot.md` + **context-meter**  

## Injected every user message (when it matches)

- **classify[…]** — route hint (intent / decision / memory / code / graph / process)  
- **Obsidian context pointers** — 0–3 index/hot lines (stopwords filtered; min 2 token hits)

## Agent loop (mandatory)

1. Read classify + pointers + hot.  
2. **INTENT** — restate verified ask + done-when in chat (do not rewrite the user prompt).  
3. If insufficient → `search_simple` / `vault_read` project Index + latest Session.  
4. Load **only** pages that change the next action.  
5. CODE write → ledger must already have `obsidian_recall` (or gate denies).  
6. Durable fact → **COMPLETE** `vault_append` (Sessions/Decisions/Learnings/sources/concepts/journals — not vibe summaries) + refresh hot.  
7. Before Done → Session with Goal / Done-when / Residual + **LAYER CHECK** table.  
8. Done → TOOLCHAIN evidence. Summary-only write-back = defect (`instructions/PROCESSING.md` COMPLETE CAPTURE).

## LAYER CHECK template (Session)

```markdown
## Layer check
| Layer | Evidence |
|-------|----------|
| Prompt | Intent + done-when restated |
| Context | vault_read hot/index; COMPLETE write-back |
| Harness | kleos-gate allow; TOOLCHAIN / cargo test |
| Loop | Session goal + Open/residual |
| Graph | [[wikilinks]] on Decisions/Sessions |
```

## Stop followups (force)

kleos-gate `stop` may require: vault flush, COMPLETE expand (stub), INTENT Session, LAYER CHECK, or house verify. Fix the named gap — do not fight the followup.

## Debug by layer (Akshay units)

| Break | Fix |
|-------|-----|
| Bad one-shot / format | Prompt (User Rules / skill) |
| Forgot decision / window junk | Context (hot / pointers / wiki) |
| No tools / no verify / gate thrash | Harness (kleos-gate / TOOLCHAIN) |
| Stopped mid-task | Loop (ship-loop / Session goals) |
| Parallel clash / orphan notes | Graph (wikilinks / shared disk state) |

## Multi-agent (0xJeyx — 6 steps)

1. Nodes only for real specialties  
2. Edges before code  
3. Shared state on disk (wiki / HANDOFF)  
4. Reviewer with teeth (`eval-pass` + TOOLCHAIN)  
5. Isolate failure by node/layer  
6. Prefer Cursor + kleos-gate + skills + vault — no CrewAI theater  

## Residual

- Classify + pointers ≠ QMD semantic recall (optional later; ASK install).  
- Stub meter is syntactic (min chars + markers) — not semantic completeness (J/Rice).  
- Green recall gate ≠ perfect curation (J).

## Law pointers

- [`LAYER-STACK.md`](LAYER-STACK.md)  
- [`AGENTIAL-CONTROL.md`](AGENTIAL-CONTROL.md)  
- Skill `obsidian-memory`  
- Vault catalog `wiki/catalogs/Layer-Stack-Threads.md`  
- Vault inventory `wiki/catalogs/Graph-Inventory.md`  
- **Full post extracts (read in vault — do not open X):**  
  `wiki/sources/akshay-Layer-Stack.md` · `wiki/sources/Sprytixl-Graph-Engineering.md` · `wiki/sources/UndefinedKi-Process-Graph.md` · `wiki/sources/0xJeyx-Six-Steps-Agent-Team.md`
