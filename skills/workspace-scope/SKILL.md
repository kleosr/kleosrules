---
name: workspace-scope
description: >-
  Scopes work safely in single-package repos, monorepos, Cargo workspaces,
  multi-app trees, and service collections. Use when choosing which package,
  app, crate, service, or shared contract a task should touch. Preserves the
  existing topology and prevents unrequested workspace-wide changes.
---

# Workspace scope

Repository topology is a constraint, not a quality verdict. Preserve it unless
the user asks for an architectural change.

## Goal

Choose the smallest coherent change boundary and verify every affected
contract without turning a local task into workspace-wide cleanup.

## Detect the topology

| Signal | Boundary candidates |
|--------|---------------------|
| One manifest / deployable | Repository root or bounded context |
| `packages/*`, pnpm/npm workspace | Package plus direct dependents |
| Cargo workspace / `crates/*` | Crate plus dependent crates |
| `frontend/`, `backend/`, `admin/` | Deployable app or service |
| Multiple services | Service plus owned contracts/data |
| Shared schema/types/client | Contract producer and consumers |

Describe what exists; do not rename it “good,” “bad,” or “debt” without
evidence of an actual cost.

## Scope lock

Before editing a multi-boundary repository, establish:

```text
SCOPE:
- primary boundary: <app/package/crate/service/context>
- affected contracts: <none or exact paths/APIs>
- dependents to inspect: <exact consumers>
- verify: <boundary-local commands; wider only when contract changed>
```

Ask one focused question only when the primary boundary cannot be inferred.

## Change rules

- Match existing package and dependency conventions.
- Start in one boundary. Cross boundaries only when the requested behavior or
  a shared contract makes it necessary.
- Grep consumers before changing public APIs, schemas, events, shared types,
  auth contracts, or database ownership.
- Do not create packages, workspace members, shared libraries, or services
  merely for organization.
- Do not split or merge repositories as incidental cleanup.
- Shared code requires real reuse and a clear owner; duplication twice is not
  enough to create a package.
- Keep domain rules with their owner, not in a generic `shared` bucket.

## Verification

1. Run the primary boundary's TOOLCHAIN or real scripts.
2. If a contract changed, verify direct consumers.
3. Run workspace-wide checks only when TOOLCHAIN requires them or the change
   truly affects the workspace graph.
4. Report touched and inspected boundaries; do not claim untouched packages
   are green.

## Architecture changes

Creating a monorepo, splitting one, adding a package, or moving ownership is a
separate architecture task. Require:

- concrete pain being solved;
- dependency and deployment impact;
- data/contract ownership;
- migration and rollback path;
- explicit user approval before implementation.

## Relation to other Skills

| Skill | Role |
|-------|------|
| agents-map | Maps current boundaries and verification paths |
| domain-architecture | Designs bounded contexts and dependency direction |
| workspace-scope | Locks the implementation and verification boundary |
| git-commit / create-pr | Ships the verified scoped change |
