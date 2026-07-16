---
name: domain-architecture
description: >-
  Designs or reviews domain boundaries, DDD layers, aggregates, invariants,
  ports, adapters, and ownership in business-rule-heavy systems. Use when the
  user asks for DDD, hexagonal architecture, bounded contexts, domain modeling,
  or organization of complex business logic. Avoid for simple CRUD and UI.
---

# Domain architecture

DDD is a response to domain complexity, not a folder template.

## Applicability gate

Use DDD structure when the code owns meaningful language, invariants,
workflows, money, legal/health rules, authorization policy, or competing
models across teams.

For straightforward CRUD, content, adapters, or UI state, preserve the
existing layered structure and add no ceremony.

## Discover before designing

1. Read AGENTS.md, TOOLCHAIN.md, manifests, entry points, schemas, and tests.
2. Identify business terms and rules already encoded in names and behavior.
3. Locate data ownership, transactions, external systems, and public contracts.
4. Name candidate bounded contexts only from evidence. Unknowns become
   questions, not invented architecture.

## Dependency model

```text
interfaces → application → domain
infrastructure → application/domain ports
domain → no framework, transport, ORM, or vendor dependency
```

| Layer | Owns |
|-------|------|
| domain | Entities, value objects, aggregates, invariants, domain events |
| application | Use cases, orchestration, authorization decisions, ports |
| infrastructure | ORM mappings, repositories, queues, APIs, filesystem |
| interfaces | HTTP/CLI/worker handlers, transport validation, presenters |

Use the repository's naming and language. These layer names are roles, not
mandatory directories.

## Modeling rules

- Aggregate boundaries follow consistency and transaction needs.
- Enforce invariants where state changes, not only in controllers.
- Value objects validate meaningful domain concepts.
- Domain services exist only for rules that belong to no entity/value object.
- Application services coordinate; they do not absorb domain policy.
- Repositories expose domain-oriented operations, not generic CRUD bases.
- Domain events communicate completed facts; commands express intent.
- One context never reaches into another context's database tables directly.
- Translate external/provider models at adapters or anti-corruption layers.

## Workspace placement

- Map each bounded context to its existing app/package/module owner.
- A bounded context does not automatically require a package or service.
- Prefer a modular monolith until independent deployment, ownership, or
  scaling creates a proven service boundary.
- Use workspace-scope before implementing cross-context work.

## Avoid

- Global `entities/`, `services/`, `repositories/` folders spanning contexts.
- `common`, `shared`, or `utils` as ownerless dumping grounds.
- Anemic models with every rule in application services.
- Framework annotations or ORM records as the domain model by default.
- Generic base repositories, factories, or event buses without real repeated
  need.
- Rewriting a working system solely to match a DDD diagram.

## Change plan

For an existing codebase:

1. Name the violated boundary or misplaced rule.
2. Choose one behavior slice.
3. Add characterization/regression coverage.
4. Move policy inward while preserving public behavior.
5. Keep adapters at the edge.
6. Verify callers and contracts.
7. Stop; migrate another slice only when requested.

## Output

Return only what changes the decision:

- domain complexity verdict: simple / moderate / DDD-justified;
- bounded contexts and owners;
- critical invariants and transactions;
- dependency violations;
- smallest viable structural change;
- verification and migration risks.
