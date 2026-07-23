---
name: agents-map
description: >-
  Build or refresh hierarchical AGENTS.md maps (and TOOLCHAIN.md when missing)
  for a codebase. Deep-init style: root map + package/app boundaries only.
  Use when the user asks to map a repo, init AGENTS, deep-init, refresh
  AGENTS.md, agents-map, or onboard the agent to a project structure.
  Not for rewriting .cursor/rules or inventing product features.
---

# Agents map (deep-init, disciplined)

On-demand only. Produces **MAP** artifacts in the repo. Does **not** rewrite
project law (`.cursor/rules/*.mdc` synced from this rules pack).

## Goal

Token-efficient orientation for agents: where to look, how to verify Done,
what must never happen here, and what to ask first. One dense root beats many
shallow files. Prefer non-discoverable facts (gotchas, landmines) over anything
the model can learn by reading source in one tool call.

## Non-negotiables

- **LAW vs MAP**: Law = `.cursor/rules/`. AGENTS.md = map only. One pointer line
  to law — never paste SAFETY essays, force-push bans, or craft dumps.
- **Evidence over fiction**: Paths, commands, hard stops only from this repo
  (tree, manifests, CI, README, existing docs). Unknown → omit or
  `unknown — verify`. Never invent stack, tests, or deploy steps.
- **No encyclopedia**: Root **~80–120 lines**. Nested **~30–60**. Tables >
  prose. No Clean Code / architecture book paste.
- **Preserve human ownership**: Keep `## Manual notes` and
  `<!-- manual -->…<!-- /manual -->` verbatim on refresh.
- **Do not commit** unless the user explicitly asks in this message.
- **Merge, don't nuke**: Existing good AGENTS.md → update sections; never
  wholesale replace large human maps (e.g. Terremoto-scale) unless asked.

## When to nest

Max **6** new nested `AGENTS.md` per run unless the user asks for more.

| Nest here | Do not nest |
|-----------|-------------|
| Existing `frontend/`, `backend/`, `admin/`, `packages/*`, `crates/*` boundaries | Every `components/`, `utils/`, `hooks/`, `__tests__/` |
| `infra/`, `deploy/` with real ops surface | Leaf UI folders |
| Separate deployable already in tree | Mirror of every src folder; **new** packages |

Default: only boundaries that **already exist**. Nested maps describe topology;
they do not justify restructuring it. Never spray AGENTS.md tree-wide.

## Procedure

### 1. Scope

Workspace root = target unless the user names another path. Multi-boundary:
list apps/packages/crates/services and their roles in a short table first.

### 2. Discover (parallel reads; do not dump into chat)

1. Tree depth 2–3 (skip `node_modules`, `dist`, `.git`, `target`, `.next`, `build`)
2. Manifests: package.json/workspaces, Cargo.toml, go.mod, pyproject, compose, Dockerfiles
3. Existing: AGENTS.md, CLAUDE.md, TOOLCHAIN.md, DEBT.md, README, `.github/workflows/`
4. Tests: `*.{test,spec}.*`, `e2e/`, `crates/*/tests`
5. Risk signals: auth, PII, prisma/drizzle, k8s, secrets examples, queues/workers,
   browser stealth, bot/Discord tokens, payments

### 3. Maturity → actions

| Signal | Action |
|--------|--------|
| TOOLCHAIN.md present | Point Done at it; do not invent parallel command lists |
| No TOOLCHAIN; has scripts/CI | **Create** TOOLCHAIN.md from real commands only |
| No verify path | Done section states gap; agent must list commands actually run |
| High blast present | **Hard stops** + **Ask first** required at root (and nested owner) |
| DEBT.md present | Link under Deep links |
| Large existing AGENTS | Surgical refresh: LAW pointer, Done, hard stops, where-to-look gaps only |

### 4. Root AGENTS.md

Apply `references/template-root.md`. Required sections:

1. LAW vs MAP + nested-read rule  
2. Overview (2–4 lines)  
3. Where to look (table)  
4. Done / verify → TOOLCHAIN  
5. Hard stops (Never) — only if risk exists; 3–8 bullets  
6. Ask first — migrations, deploy, secrets apply, live automation, bulk data ops  
7. Deep links (real paths only; include DEBT.md if present)  
8. Manual notes (preserve or stub)

**Hard stops content rule (Addy filter):** prefer landmines the agent cannot
see from one file open (moderation defaults, dual gateway, staging-only PRs,
session vault paths). Skip generic "write good code".

**Human pass:** after writing hard stops from evidence, flag 1–3 candidate
landmines the repo does not document and ask the user to confirm or fill
Manual notes — do not invent them.

### 5. TOOLCHAIN.md if missing

`references/template-toolchain.md`. Copy-paste real scripts/CI only.
Existing TOOLCHAIN → do not overwrite; report drift if scripts diverged.

### 6. Nested maps

`references/template-nested.md`. Parent link correct. Local Done only if
commands differ. Hard stops only if this package owns the risk.

### 7. Report

| File | created / updated / skipped | Why |
|------|-----------------------------|-----|
| … | … | … |

Residual gaps: no tests, no CI, undocumented domain risk. No PR/commit unless
asked.

## Refresh mode

Re-discover → update moved paths → preserve Manual notes → delete nested only
if dir gone or user asked → do not grow nested count without need.

## Anti-patterns

- Unprompted regen every session  
- Copying agent.mdc into AGENTS.md  
- Fake `npm test` when no test script  
- Nested file per folder "for completeness"  
- Done claimed without TOOLCHAIN or honest gap  
- Hooks installation as part of this skill  

## Harness position

| Layer | Owns |
|-------|------|
| this rules pack `*.mdc` | Always-on law (synced); topology preservation |
| This skill | Warm MAP + TOOLCHAIN bootstrap |
| workspace-scope | Implementation and verification boundary |
| domain-architecture | Bounded contexts and domain dependency direction |
| formulary | Grok 4.5 prompt discipline |
| git-commit / create-pr / frontend-design | Ops protocols |
| Hooks | Out of scope here |

## Templates

- `references/template-root.md`
- `references/template-nested.md`
- `references/template-toolchain.md`
