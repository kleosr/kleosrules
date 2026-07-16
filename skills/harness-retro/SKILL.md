---
name: harness-retro
description: >-
  Turn a repeated agent failure into a harness fix (rule, skill, TOOLCHAIN,
  or hook) instead of only patching product code. Use when the same mistake
  recurs, the user says harness retro, or keep-rate drops for a pattern.
---

# Harness retro

Hashimoto: every repeated mistake → fix the harness, not only the repo.

## Workflow

1. State the **symptom** (what the agent did wrong) with evidence.
2. Classify layer:
   - `agent.mdc` / craft law
   - Skill missing or weak
   - TOOLCHAIN / CI gap
   - Hook (enforceable SAFETY)
   - Map (`AGENTS.md`) missing
3. Propose the **smallest** harness change (one layer). Prefer Skill over
   always-on prose; prefer TOOLCHAIN over soft reminders; prefer hook only
   for hard SAFETY.
4. If authorized: apply the harness change in `Documentos/rules`, run
   `scan-and-sync.sh`, verify-sync green.
5. Optionally note keep-rate: did humans revert similar agent commits recently?

## Anti-patterns

- Thickening User Rules with craft essays
- Adding a 5th always-on `.mdc`
- Reintroducing Spec Kit or sticky epic state
- Fixing only the product bug when the agent will repeat the process error

## Report

Symptom → layer → harness change → sync evidence → residual risk.
