# Release — kleosrules v9.2.2

## Skills / architecture / lean

Full Option C skill set for this pack:

- **Native Lean:** `ponytail`, `lean-code`, `vernacular`, `unconditional-counterexample`
- **Architecture:** `architecture-fitness`, `improve-codebase-architecture`, plus domain/agents/wiring helpers
- **Frontend:** `design-taste-frontend`, `ui-ux-audit`, …
- **Product/voice:** `cursor-research`, `benln-write`

`config/skills.txt` is the install manifest. User Rules SKILL ROUTING points at the new entries.

## Install

```bash
FORCE_SKILLS=1 bash install.sh
# paste user-rules/USER-RULES.paste.txt if User Rules are stale
```
