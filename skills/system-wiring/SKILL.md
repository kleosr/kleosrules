---
name: system-wiring
description: >-
  Traces, designs, or verifies end-to-end communication across UI, APIs,
  application/domain logic, persistence, providers, events, queues, workers,
  and consumers. Use when integrating components or diagnosing missing,
  duplicated, stale, or broken communication between boundaries.
---

# System wiring

Complete the requested behavior end to end; do not connect unused code.

## Trace

```text
entry → transport validation → use case → domain rule → port → adapter
      → persistence/provider/event → consumer/response → UI or caller
```

1. Name the initiating actor, expected outcome, and primary owner.
2. Map only observed hops: entry points, calls, schemas, events, state changes,
   and consumers. Unknown links are gaps, not assumptions.
3. Identify the source of truth and transaction/consistency boundary.
4. Record each contract's producer, consumer, validation, version, and failure
   behavior.
5. Grep dependents before changing shared APIs, schemas, events, or types.

## Correctness checks

- Validate and authorize at trust boundaries; domain invariants stay with the
  domain owner.
- Preserve correlation/context across hops without logging secrets.
- Define timeout, cancellation, retry, ordering, and idempotency only where
  asynchronous or remote behavior requires them.
- Make partial failure explicit; no swallowed errors or false success.
- Keep adapters responsible for vendor/transport translation.
- Add contract tests at changed boundaries and one integration test for the
  critical path when the repository supports it.

## Change rules

- Use `workspace-scope` to lock affected boundaries and verification.
- Wire the smallest complete vertical slice; no generic bus, shared package,
  base client, or abstraction before real repetition.
- Zero consumers → delete or leave unwired; never invent a caller.
- Diagnose-only requests return the broken/missing hop without edits.

## Report

Show the verified flow, broken or added hop, contracts affected, failure
semantics, and local/consumer verification. Do not claim untested hops work.
