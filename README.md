# kleosrules

My Cursor rules pack: short patches, no prose comments, vernacular when the
repo has one, hooks that block the usual screw-ups.

Option C / Native Lean is the policy inside. This repo is how I install and
sync it.

## Layout

```
.
├── README.md              # this file
├── LICENSE                # MIT
├── AGENTS.md              # map of this pack
├── package.json
├── install.sh             # one-shot → ~/.cursor
├── user-rules/            # paste into Cursor User Rules
│   ├── USER-RULES.paste.txt   # Option C V9.2
│   └── option-c-core.mdc      # disk mirror (alwaysApply: false)
├── project-rules/         # synced into each repo's .cursor/rules
│   ├── agent.mdc
│   ├── native-lean-autoload.mdc
│   ├── types.mdc
│   ├── testing.mdc
│   └── debugging.mdc
├── hooks/                 # mechanical gates
├── skills/                # on-demand Cursor skills
├── config/                # skills.txt, scan.*, retired.*
├── scripts/               # sync / verify / scan
├── lib/                   # discovery helpers
└── docs/                  # RELEASE, TOOLCHAIN, USER-RULES guide
```

## Install

```bash
cd /path/to/kleosrules
FORCE_SKILLS=1 bash install.sh
```

1. Cursor → Settings → Rules → User Rules
2. Paste all of `user-rules/USER-RULES.paste.txt`
3. Start a new agent chat
4. Settings → Hooks — confirm they loaded

For an app repo that needs its own naming contract:

```bash
mkdir -p .cursor/rules
cp skills/vernacular/TEMPLATE.md .cursor/rules/vernacular.mdc
```

Fleet sync (optional):

```bash
# edit config/scan.roots first
bash scripts/scan-and-sync.sh
```

## Obedience stack

| Layer | Path |
|-------|------|
| Policy | `user-rules/USER-RULES.paste.txt` |
| Autoload | `project-rules/native-lean-autoload.mdc` |
| Vernacular | per-repo `.cursor/rules/vernacular.mdc` |
| Hooks | `hooks/` |
| Skills | `skills/ponytail`, `skills/vernacular`, … |

User Rules alone are not enough. Install the whole pack.

## Skills

Manifest: `config/skills.txt`. Install links them into `~/.cursor/skills`.

| Group | Skills |
|-------|--------|
| Native Lean | `ponytail`, `lean-code` (alias), `vernacular`, `unconditional-counterexample` |
| Architecture | `architecture-fitness`, `improve-codebase-architecture`, `domain-architecture`, `agents-map`, `workspace-scope`, `system-wiring`, `codebase-memory` |
| Frontend | `design-taste-frontend`, `ui-ux-audit`, `frontend-design`, `design-tokens`, `ui-structure`, `no-hardcode` |
| Ship / harness | `git-commit`, `create-pr`, `bug-hunt`, `formulary`, `ship-loop`, `session-handoff`, `eval-pass`, `harness-retro`, `grill-me`, `humanizer` |
| Product / voice | `cursor-research`, `benln-write` |

From User Rules: lean → `/ponytail`; dialect → `/vernacular`; fitness → `/architecture-fitness`; deepening report → `/improve-codebase-architecture`; breakthrough hunt → `/unconditional-counterexample`.
