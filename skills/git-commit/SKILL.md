---
name: git-commit
description: >-
  Create a git commit using this repo's safety protocol and message style.
  Use when the user asks to commit, create a commit, or save staged work
  with git. Not for push, amend-unless-requested, or force operations.
---

# Git commit

Only create commits when the user explicitly asks in the current message.
If unclear, ask first. Obey project `agent.mdc` SAFETY (earlier wins).

## Safety (non-negotiable)

- NEVER update git config
- NEVER force-push. Ever. If asked: refuse and warn; do not execute.
  Same for hard reset and other destructive/irreversible git unless the
  user explicitly requests that exact operation (force-push stays banned).
- NEVER skip hooks (`--no-verify`, `--no-gpg-sign`, etc.) unless asked
- Avoid `git commit --amend` unless ALL hold:
  1. User explicitly requested amend, OR commit succeeded but a
     pre-commit hook auto-modified files that must be included
  2. HEAD was created by you in this conversation
     (`git log -1 --format='%an %ae'`)
  3. Commit has NOT been pushed (`git status` shows branch ahead)
- If commit FAILED or was REJECTED by a hook: NEVER amend — fix and
  create a NEW commit
- If already pushed: NEVER amend (amend after push implies force-push)
- NEVER use interactive flags (`-i`, `git add -i`, `rebase -i`)
- Do not commit secrets (`.env`, credentials.json, etc.); warn if asked
- Do not push unless the user explicitly asks

## Procedure

1. In parallel:
   - `git status` (untracked + staged)
   - `git diff` and `git diff --staged`
   - `git log -5 --oneline` (match message style)
2. Draft a concise 1–2 sentence message focused on **why**, not what.
   Reflect nature accurately: add / update / fix / refactor / test / docs.
3. Stage relevant files only. Then commit via HEREDOC:

```bash
git commit -m "$(cat <<'EOF'
Commit message here.

EOF
)"
```

4. `git status` after commit to verify success.
5. If hook fails: fix, then NEW commit (no amend).
6. If nothing to commit: do not create an empty commit.

Do not explore the codebase beyond these git commands for the commit
itself unless needed to avoid committing secrets.
