# Cursor Source Seed Catalog

**Navigation aid only — never evidence.** Re-fetch and verify every linked source in the current session before citing. Routing hints and historical notes are starting points, not facts.

URLs marked `[verified YYYY-MM-DD]` were confirmed live on that date. Everything else is a best-guess seed or a search template — expect paths to move. **Only update a verification date after actually fetching the URL.** Bumping the date without fetching turns this file into a liability: it launders stale paths as fresh ones.

## Stable entry points

These top-level URLs rarely move; deep paths below them do.

- Docs home: https://cursor.com/docs `[verified 2026-07-19]` — covers Agent, Rules, Skills, MCP, Hooks, CLI, Models, Tab, Cloud Agents, Enterprise
- Help center (account/billing/usage, also deeper customization pages): https://cursor.com/help — e.g. `/help/account-and-billing/pricing`, `/help/models-and-pricing/available-models`, `/help/customization/rules`, `/help/customization/skills` `[verified 2026-07-19]`
- Pricing (marketing): https://cursor.com/pricing `[verified 2026-07-19]` — cross-check against docs; numbers can disagree with `/docs/account/teams/pricing`
- Blog: https://cursor.com/blog
- Changelog: https://cursor.com/changelog `[verified 2026-07-19]`
- Forum: https://forum.cursor.com
- GitHub org: https://github.com/getcursor
- Privacy policy: https://cursor.com/privacy · Terms: https://cursor.com/terms
- Trust Center: https://trust.cursor.com `[verified 2026-07-19]`
- Install/download: https://cursor.com/install
- Legacy docs domain (may appear in search, redirects): https://docs.cursor.com

## Verified deep paths

| Topic | URL | Verified |
|---|---|---|
| Agent overview | https://cursor.com/docs/agent/overview | 2026-06-14 |
| Hooks (agent + tab + workspace lifecycle) | https://cursor.com/docs/hooks | 2026-07-19 |
| Plugins reference (hooks.json, plugin layout) | https://cursor.com/docs/reference/plugins | 2026-06-14 |
| Third-party hooks (Claude Code / Codex compat) | https://cursor.com/docs/reference/third-party-hooks | 2026-06-14 |
| Skills | https://cursor.com/docs/skills | 2026-07-19 |
| Rules | https://cursor.com/docs/rules | 2026-07-19 |
| MCP (editor) | https://cursor.com/docs/mcp | 2026-06-14 |
| MCP (CLI: `agent mcp ...`) | https://cursor.com/docs/cli/mcp | 2026-06-14 |
| Models & pricing | https://cursor.com/docs/models-and-pricing | 2026-07-19 |
| Team pricing / seats | https://cursor.com/docs/account/teams/pricing | 2026-07-19 |
| Tab / autocomplete | https://cursor.com/docs/tab/overview | 2026-07-19 |
| Bugbot | https://cursor.com/docs/bugbot | 2026-07-19 |
| Cloud Agents — Automations | https://cursor.com/docs/cloud-agent/automations | 2026-07-19 |
| Cloud Agents — My Machines | https://cursor.com/docs/cloud-agent/my-machines | 2026-06-14 |
| Cloud Agents — Self-Hosted Pool | https://cursor.com/docs/cloud-agent/self-hosted-pool | 2026-06-14 |
| Cloud Agents — Self-Hosted (Kubernetes) | https://cursor.com/docs/cloud-agent/self-hosted-k8s | 2026-06-14 |
| Cloud Agents — Self-Hosted (Cloud Run) | https://cursor.com/docs/cloud-agent/self-hosted-cloud-run | 2026-06-14 |
| Cloud Agents API (v1 agents/runs/stream) | https://cursor.com/docs/cloud-agent/api/endpoints | 2026-07-19 |
| CLI — using Agent non-interactively | https://cursor.com/docs/cli/using | 2026-07-19 |
| REST API overview (Admin/Analytics/AI Code Tracking/Bugbot API/Cloud Agents API/SDK) | https://cursor.com/docs/api | 2026-07-19 |
| TypeScript SDK (`@cursor/sdk`) | https://cursor.com/docs/sdk/typescript | 2026-07-19 |
| Python SDK (`cursor-sdk` / `cursor_sdk`) | https://cursor.com/docs/sdk/python | 2026-07-19 |
| Evals (SDK harness) | https://cursor.com/docs/evals | 2026-07-19 |
| Enterprise | https://cursor.com/docs/enterprise | 2026-07-19 |
| Slack integration | https://cursor.com/docs/integrations/slack | 2026-07-19 |
| Composer 2.5 | https://cursor.com/docs/models/cursor-composer-2-5 | 2026-07-19 |
| Composer 2 (may redirect / alias) | https://cursor.com/docs/models/cursor-composer-2 | 2026-07-19 |

Note on "Composer": as of 2026 the name refers to Anysphere's model family (Composer 2 / 2.5). The 2024-era `/docs/composer/overview` and `/docs/chat/overview` pages are gone — that functionality lives under `/docs/agent/*`. Don't seed fetches at the old paths. Prefer `/docs/models/cursor-composer-2-5` for current model docs.

## Unverified seeds (fetch before trusting; search if 404)

- Integrations index: search `site:cursor.com/docs integrations` (GitHub, Slack, GitLab, Linear, Jira, Bitbucket)
- Keyboard shortcuts: search `site:cursor.com/docs shortcuts` — path has moved before; do not assume
- Dashboard (account-specific, navigation hint for the user only): https://cursor.com/dashboard (billing, usage, settings, cloud-agents, admin under it)
- Support contact: hi@cursor.com `[seen on cursor.com/pricing 2026-06-09]`
- Status / incident pages: search cursor.com for "status" (do not invent a status URL)

## Forum

Home: https://forum.cursor.com — categories (paths shift; browse from home or search if a direct link 404s):

- **Announcements** — official updates
- **Support / Help** — user Q&A
- **Bug Reports** — reproducible issues with staff responses
- **Discussions / Guides** — workflows (verify against docs)
- **Ideas** — feature requests; **NOT shipped features**
- **Account & Billing** — billing questions

### Staff search hints

Authority comes from the **staff badge/role on the thread**, never from this list. These usernames have appeared in official contexts and may help searches; they go stale:

`mohitjain, deanrie, kevinn, danperks, colin, michaeltricht, amar, sualehasif`

Prefer `site:forum.cursor.com "cursor team" <topic>` over hardcoding names.

## Query templates

Pass as natural-language `query` to `web_search_exa` (or to the fallback web search):

```
Cursor official documentation about <topic> site:cursor.com/docs
Cursor help center <billing/account/customization topic> site:cursor.com/help
Cursor forum staff answer about <topic> site:forum.cursor.com
Cursor forum bug report <error or feature> site:forum.cursor.com inurl:bug-report
Cursor forum workaround <topic> site:forum.cursor.com "workaround" OR "this worked for me"
Cursor changelog entry for <feature> site:cursor.com/changelog
Cursor blog announcement <feature> site:cursor.com/blog
Cursor models and pricing plan details site:cursor.com/docs/models-and-pricing
Cursor team seat pricing site:cursor.com/docs/account/teams/pricing
Cursor enterprise <feature> site:cursor.com/docs/enterprise
Cursor open source <topic> site:github.com/getcursor
```

Recent changes: add `after:2025-06-01` (adjust the date). Error messages: search the exact quoted string.

### Community (supplement only — never sole source, always label community-reported)

```
site:reddit.com/r/cursor <topic>
site:news.ycombinator.com cursor <topic>
"<feature name>" cursor workaround OR fix OR bug
```

Discord is generally not indexable — do not rely on search reaching it.

## Not official

Treat as unverified unless cross-checked against docs: random GitHub repos, Reddit/YouTube/SEO blogs mirroring docs, AI-generated articles about Cursor, third-party marketplace extensions, forum Ideas posts — and this file itself.
