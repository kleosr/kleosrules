---
name: create-pr
description: >-
  Create a GitHub pull request with gh: push if needed, summarize the
  full branch diff, open the PR. Use when the user asks to open/create
  a PR or pull request. Not for merge, force-push, or babysitting CI.
---

# Create pull request

Use `gh` via the Shell tool for all GitHub PR work.
Obey project `agent.mdc` SAFETY (earlier wins).

## Safety

- NEVER update git config
- NEVER force-push. Ever. If asked: refuse and warn; do not execute.
- Do NOT use TodoWrite or Task for this workflow
- Push only as needed to open/update the PR (`git push -u` / fast-forward)

## Procedure

1. In parallel:
   - `git status`
   - `git diff` and `git diff --staged`
   - Check whether the branch tracks a remote and is up to date
   - `git log` and `git diff [base-branch]...HEAD` (full history since
     divergence — all commits in the PR, not only the latest)
2. Draft title + summary from the full change set.
3. Sequentially:
   - Create branch if needed
   - `git push -u origin HEAD` if the branch is not on the remote
   - Create PR with HEREDOC body:

```bash
gh pr create --title "the pr title" --body "$(cat <<'EOF'
## Summary
- …

## Test plan
- [ ] …

EOF
)"
```

4. Return the PR URL when done.
