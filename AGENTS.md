# AGENTS.md — map of this pack

## Law vs map

- **Law:** `user-rules/USER-RULES.paste.txt` + `project-rules/*.mdc` + `hooks/`
- **This file:** navigation only

## Where

| Need | Path |
|------|------|
| Paste User Rules | `user-rules/USER-RULES.paste.txt` |
| Option C mirror | `user-rules/option-c-core.mdc` |
| Project craft | `project-rules/` |
| Install | `install.sh` |
| Hooks | `hooks/` |
| Skills list | `config/skills.txt` |
| Lean / vernacular | `skills/ponytail`, `skills/vernacular` |
| Architecture | `skills/architecture-fitness`, `skills/improve-codebase-architecture` |
| Breakthrough hunt | `skills/unconditional-counterexample` |
| Sync / verify | `scripts/` |
| Docs | `docs/` |

## Commands

```bash
FORCE_SKILLS=1 bash install.sh
bash scripts/scan-and-sync.sh
bash scripts/verify-sync.sh
```
