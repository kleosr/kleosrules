# kleosr

Cursor harness by **kleosr**: User Rules, always-on companions, skills, and the Rust
gate (`kleos-gate`) that blocks the usual agent screw-ups.

Map: [`AGENTS.md`](AGENTS.md). Done recipe: [`docs/TOOLCHAIN.md`](docs/TOOLCHAIN.md).
Release notes: [`docs/RELEASE.md`](docs/RELEASE.md). Paste guide: [`docs/USER-RULES.md`](docs/USER-RULES.md).

## What you get

| Layer | Where |
| --- | --- |
| User Rules (Master Mind) | [`user-rules/USER-RULES.paste.txt`](user-rules/USER-RULES.paste.txt) |
| Always-on companions | [`project-rules/`](project-rules/) → synced into `.cursor/rules` |
| Mechanical gate | [`hooks/bin/kleos-gate`](hooks/bin/kleos-gate) + [`hooks/policy/`](hooks/policy/) |
| Skills | [`skills/`](skills/) via [`config/skills.txt`](config/skills.txt) → `~/.cursor/skills` |
| Fleet sync / verify / bench | `kleos-gate` CLI (`install`, `sync`, `verify`, `bench`) |

No Python and no pack shell. Tooling is Rust-only.

## Setup

Needs Rust (`cargo`) once to build, or use the checked-in binary under `hooks/bin/`.

```bash
git clone <this-repo> && cd rules
cargo build --release --manifest-path hooks/kleos-gate/Cargo.toml
mkdir -p hooks/bin
cp -f hooks/kleos-gate/target/release/kleos-gate hooks/bin/kleos-gate
FORCE_SKILLS=1 hooks/bin/kleos-gate install
```

Then:

1. Cursor → Settings → Rules → User Rules
2. Paste all of `user-rules/USER-RULES.paste.txt`
3. Start a **new** agent chat
4. Settings → Hooks — confirm `kleos-gate` loaded

Per-app naming contract (optional):

```bash
mkdir -p .cursor/rules
cp skills/vernacular/TEMPLATE.md .cursor/rules/vernacular.mdc
```

Fleet (optional — edit `config/scan.roots` first):

```bash
hooks/bin/kleos-gate sync
hooks/bin/kleos-gate verify
```

## Layout

```
.
├── AGENTS.md
├── README.md
├── LICENSE
├── package.json
├── user-rules/
├── project-rules/
├── hooks/
├── skills/
├── config/
├── docs/
└── .github/
```

## Verify

```bash
cd hooks/kleos-gate && cargo test && cargo build --release
hooks/bin/kleos-gate verify
hooks/bin/kleos-gate bench
hooks/bin/kleos-gate gate-diff
```

Full house: [`docs/TOOLCHAIN.md`](docs/TOOLCHAIN.md).

## Skills

Install links dirs from `config/skills.txt` into `~/.cursor/skills`.

| Group | Skills |
| --- | --- |
| Native Lean | `ponytail`, `lean-code`, `vernacular`, `unconditional-counterexample` |
| Architecture | `architecture-fitness`, `improve-codebase-architecture`, `domain-architecture`, `agents-map`, `workspace-scope`, `system-wiring`, `codebase-memory` |
| Frontend | `design-taste-frontend`, `ui-ux-audit`, `frontend-design`, `design-tokens`, `ui-structure`, `no-hardcode` |
| Ship | `git-commit`, `create-pr`, `bug-hunt`, `formulary`, `ship-loop`, `session-handoff`, `eval-pass`, `harness-retro`, `grill-me`, `humanizer` |
| Voice | `cursor-research`, `benln-write` |

Route from User Rules: lean → `/ponytail`; dialect → `/vernacular`; fitness → `/architecture-fitness`; breakthrough → `/unconditional-counterexample`.

## Doctrine (short)

MUST-NEVER/M is gate-backed (`kleos-gate`). Soft skills/companions are J-authority when
routed; they never waive M or fight a live deny. Lean meter = size roofs only
([`docs/evals/LEAN-SIZE-QUALITY-PSTAR.md`](docs/evals/LEAN-SIZE-QUALITY-PSTAR.md)).
Green TOOLCHAIN ≠ ∀ semantic proof
([`docs/MECHANICAL-INCOMPLETENESS.md`](docs/MECHANICAL-INCOMPLETENESS.md)).

Deeper stack: [`docs/AGENTIAL-CONTROL.md`](docs/AGENTIAL-CONTROL.md) ·
[`docs/MODEL-SPEC.md`](docs/MODEL-SPEC.md) ·
[`docs/AGENTIC-GAUNTLET.md`](docs/AGENTIC-GAUNTLET.md).
