---
name: cursor-research
description: >-
  Answers Cursor product questions from verified live sources only — features,
  pricing, plans, billing, bugs, UI behavior, layouts, integrations, CLI/SDK/API,
  Cloud Agents, Automations, Skills, Hooks, MCP, Bugbot, Composer, enterprise,
  privacy/security, and support status. Use whenever the user asks what Cursor
  can do, how a Cursor feature works, what something costs, which plan includes
  a feature, whether a bug is known, or anything else about the Cursor product
  itself — even if they don't say "research" or "verify". Do not use for general
  coding tasks that merely happen inside Cursor.
version: 2.2.0
---

# Cursor Research (verified sources only)

Answer Cursor **product** questions from sources fetched in this session. Training data is a hypothesis, not evidence — the product changes monthly, so a memorized answer about features, pricing, or behavior is a guess wearing a confident tone. This skill exists to replace that guess with a fetch.

Scope: the Cursor product. A user debugging their own code inside Cursor is not a product question; a user asking why Cursor's agent behaves a certain way is. Mentions of Composer, Bugbot, Automations, Cloud Agents, Skills, Hooks, MCP, CLI, or SDK almost always are product questions — fetch.

## Non-negotiables

Each rule appears once. They all reduce to the same idea: claims need fetched evidence.

1. **Fetch before you claim.** Every Cursor-specific factual claim (features, pricing, plans, availability, commands, UI behavior, security, privacy, bug status) must come from a page fetched this session.
2. **Never fabricate.** No invented doc pages, forum threads, quotes, pricing, plan gates, CLI flags, env vars, API endpoints, or URLs. If you cannot verify something, say so under UNVERIFIED — a labeled gap is more useful than a confident invention.
3. **A URL is not verified unless the fetch succeeded.** `web_fetch_exa` must return the page body (HTTP 200, real content — not a login page, 404 stub, or domain-park). A search-result snippet is a hint, not a citation. If a fetch returns 401/403 or a sign-in wall, label the claim **unverified (auth-walled)** and do not paraphrase from the snippet. Never write a URL into an answer that you have not actually opened this session.
4. **Cite non-obvious claims inline.** One citation may cover a sentence or small cluster of claims from the same source. Skip citations on trivial connective prose. Paraphrase general explanations; quote exact wording for pricing, plan gates, staff bug status, beta status, limitations, and security/privacy commitments — these are the claims where paraphrase drift causes real damage.
5. **Date-stamp volatile facts.** Volatile set: pricing, plan gates, plan requirements, model lists, default models, beta status, feature flags, rate limits, supported integrations list, and any "available in Cursor X.Y and above" claim. Write "verified YYYY-MM-DD" next to them. For sources older than ~6 months on fast-moving topics, note "older source; may be stale". Prefer changelog/blog when docs predate a recent announcement.
6. **Tie behavioral claims to a Cursor version when the source does.** Docs often gate behavior on a version (e.g. "Auto-review mode, Cursor 3.6+"). If the page says "X, in version Y", your answer says "X (Cursor Y+)" — not just "X". Drop the version only when the source states the behavior unconditionally. **If the user names the version they're on** ("I'm on Cursor 3.4"), treat it as a filter: prefer the docs/changelog current at that version, and flag any claim that only checks out on a newer build as **unverified for that version** rather than asserting it works.
7. **Staff authority lives on the thread, not the username.** A claim is staff-confirmed only if the staff badge/official role is visible on that thread. Usernames go stale and get impersonated — search hints live in [sources.md](sources.md), proof lives on the thread.
8. **Account-specific data is user-provided only.** Subscription status, invoices, usage, refunds, seats, team admin settings: verifiable only from material the user supplies (redacted screenshots or excerpts). Never ask for passwords, tokens, API keys, or dashboard exports. Dashboard URLs are navigation hints for the user, not evidence.
9. **sources.md is a navigation aid, never evidence.** Re-fetch any linked page in the current session before citing it. Its routing hints and historical notes are starting points, not facts.
10. **Out of scope → say so.** Not a Cursor product question, or live verification impossible: state that plainly and offer only clearly-labeled general guidance.

## Confidence labels

Use inline or in the UNVERIFIED section:

| Label | Meaning |
|-------|---------|
| **Verified official** | Current docs, official blog/changelog, or staff post (badge verified on thread) |
| **Official but possibly stale** | Older docs/blog with no newer confirmation |
| **Community-reported** | Forum/Reddit/community thread, not staff-confirmed |
| **User-provided** | Based on the user's account, dashboard, screenshot, or supplied link |
| **Unverified** | Not found after reasonable search |
| **Unverified (auth-walled)** | Fetch returned a login/paywall; snippet seen but not confirmed |
| **Out of scope** | Not a Cursor product question or cannot be answered safely |

## Depth (pick one before searching)

**Quick** — doc link, shortcut, single fact, "where is X documented?". Fetch 1–2 authoritative pages, stop when answered. Output: direct answer + 1–2 citations.

**Standard** — normal product behavior, setup, "how does X work?". Fetch the relevant docs in full; add forum/blog/changelog only if docs are silent, conflicted, or the topic is recent. Stop after docs + one secondary source unless still uncertain.

**Deep** — pricing/plan gates, bugs/workarounds, security/privacy, layout-sensitive UI, source conflicts, or broad "everything about X". Follow the deep checklist below; output the full format with Shared Fact Sheet + Sources.

Do not run the deep workflow for simple questions — that is process theater, not rigor.

## Source priority (highest → lowest)

| Level | Source | Authority |
|-------|--------|-----------|
| 1 | `cursor.com/docs/*`, `cursor.com/help/*` | Intended product behavior |
| 2 | `cursor.com/blog/*`, `cursor.com/changelog/*` | Releases, roadmap, announcements |
| 3 | `forum.cursor.com` (staff badge verified) | Official clarifications, bug status |
| 4 | `forum.cursor.com` (community) | User reports, workarounds — **community-reported** |
| 5 | `github.com/getcursor/*` | Open-source components, issues |
| 6 | User-provided links/excerpts | Account-specific context — **user-provided** |
| 7 | Third-party (Reddit, HN, YouTube, blogs) | Supplement only, never sole source |

**Conflict resolution:** compare date (newer usually wins for current behavior), author (staff badge > community), and context (docs = intended behavior; staff/forum = actual behavior today; changelog when docs look stale or a feature was renamed). **Pricing/plan pages often disagree with each other** (marketing `cursor.com/pricing` vs `/docs/models-and-pricing` vs `/docs/account/teams/pricing`) — fetch at least two, quote numbers with source + date, and if they conflict report both rather than averaging. If sources still conflict, **report both sides with links and date stamps, then stop** — do not pick a winner you cannot defend. "Docs say X [link], staff/users report Y [link]; as of YYYY-MM-DD the discrepancy is unresolved." High forum vote counts are popularity, not proof.

## Tools

**Primary: Exa** (`web_search_exa`, `web_fetch_exa`). Namespace/server name varies by Cursor build (often `plugin-exa-exa`). Discover the live schema before calling — do not invent parameter names from memory.

| Step | Tool | Notes |
|------|------|-------|
| Find | `web_search_exa` | Natural-language query describing the ideal page + `site:` filter. `numResults` (schema default 10): ~3–5 quick · 5–10 standard · 10–20 deep. Above ~20 wastes context. |
| Read | `web_fetch_exa` | Always fetch full pages before citing — snippets are not verification. **Batch URLs into one call** (`urls: [...]`). `maxCharacters` (schema default 3000): 5000–10000 docs/pricing/policy · 3000–5000 forum threads · 1000–2000 quick checks. Bump higher for pricing/SDK reference pages. |

Example arguments (invoke via whatever MCP/dynamic tool surface this session exposes):

```
web_search_exa: { query: "Cursor official documentation about <topic> site:cursor.com/docs", numResults: 5 }
web_fetch_exa:  { urls: ["https://cursor.com/docs/<page>"], maxCharacters: 8000 }
```

If a call errors on parameters, re-read the tool schema and retry once — do not guess values.

**Fallback:** if Exa is unavailable, tell the user to enable it in **Settings → Tools & MCP** (or run `/exa-setup`), then use built-in web search/fetch, and say the answer was verified via fallback. If all live web tools are down, the answer **cannot be verified live** — offer only general guidance labeled unverified.

Query templates for every source type live in [sources.md](sources.md). For deep research, run 2–3 searches with different phrasings (docs, forum, changelog) and merge.

## Deep checklist (complex questions only — keep progress private)

```
- [ ] Restate the question; list sub-questions
- [ ] Search official docs (2–3 phrasings); BATCH-fetch top 3–8 URLs in one web_fetch_exa call
- [ ] UI/layout question → Layout verification (below)
- [ ] Pricing/plans → playbook below
- [ ] Bug/workaround → playbook below
- [ ] API/SDK/CLI → fetch the relevant reference pages
- [ ] Docs silent or conflicted → forum (staff first), then blog/changelog
- [ ] Open-source component → github.com/getcursor
- [ ] Still unreported → optional community search; label community-reported
- [ ] Unresolvable conflict between two authoritative sources → report both, stop
- [ ] Write answer with confidence labels + inline citations
- [ ] Build Shared Fact Sheet
```

## Stop conditions

- Enough authoritative evidence answers the question — do not chase redundant sources.
- Simple question satisfied by docs + at most one secondary source.
- 2–3 failed searches on a niche topic → mark **Unverified**, list what was checked, stop. Before giving up: broaden the query, check the changelog (renames/merges), check forum Ideas (if it's a request, it doesn't exist yet), check GitHub issues, try the error message verbatim, try a date filter (`after:2025-06-01`).
- Two authoritative sources that disagree, with no tiebreaker → report both sides with links and date stamps, stop. Do not silently pick one.
- Community threads repeating the same unconfirmed report → cite one representative thread, not every duplicate.

## Topic routing

Paths verified at varying dates (see [sources.md](sources.md) for per-URL date stamps; some are older than today). Re-fetch before citing — Cursor moves these.

| User topic | Primary sources | Secondary |
|---|---|---|
| Agent, chat, tools, prompting | `/docs/agent/overview`, `/help/customization/...` | Forum Discussions |
| Agent hooks (session, tool, shell, MCP, edit, prompt events) | `/docs/hooks`, `/docs/reference/plugins` (hooks.json format) | Forum Guides |
| Skills | `/docs/skills`, `/help/customization/skills` | Forum Guides |
| Rules, AGENTS.md, CLAUDE.md, project/user/team rules | `/docs/rules`, `/help/customization/rules` | Forum Guides |
| MCP (servers, tools, prompts, resources, roots, elicitation, apps, permissions) | `/docs/mcp`, `/docs/cli/mcp` | Forum Support |
| Plugins (bundled rules+skills+agents+commands+MCP+hooks) | `/docs/reference/plugins` | Forum |
| Third-party / Claude Code / Codex compatibility | `/docs/reference/third-party-hooks` | Forum |
| Cloud Agents (managed runtime) | `/docs/cloud-agent/*` | Blog, changelog |
| Automations (scheduled + event triggers: GitHub, GitLab, Slack, Linear, webhook, cron) | `/docs/cloud-agent/automations` | Blog |
| My Machines (personal worker on your laptop/devbox) | `/docs/cloud-agent/my-machines` | — |
| Self-Hosted Pool / Enterprise fleet | `/docs/cloud-agent/self-hosted-pool`, `/self-hosted-k8s`, `/self-hosted-cloud-run` | Blog |
| Cloud Agents REST API (v1 agents/runs/stream) | `/docs/cloud-agent/api/endpoints` | API Overview |
| CLI (`agent` non-interactive, modes, worktrees, ACP) | `/docs/cli`, `/docs/cli/using`, `/docs/cli/mcp` | Forum |
| TypeScript SDK (`@cursor/sdk`, `Agent.create/send/resume/prompt`) | `/docs/sdk/typescript` | Forum, `/docs/evals` |
| Python SDK (`cursor-sdk` / `cursor_sdk`) | `/docs/sdk/python` | Forum |
| REST API overview (Admin, Analytics, AI Code Tracking, Bugbot API — Enterprise gated; Cloud Agents API — Beta; SDKs) | `/docs/api` | Forum |
| Models, cost, plans, default models | `/docs/models-and-pricing`, `cursor.com/pricing`, `/docs/account/teams/pricing`, `/help/models-and-pricing/*` | Changelog, forum billing |
| Enterprise, SSO, SCIM, audit, Admin API | `/docs/enterprise`, `/docs/api` (Admin API) | Trust Center |
| Integrations (Slack, GitHub, GitLab, Linear, Jira, Bitbucket, Bugbot) | `/docs/integrations/*`, `/docs/bugbot` | Forum Support, changelog |
| Privacy, security, data, trust | `cursor.com/privacy`, `cursor.com/terms`, `trust.cursor.com`, `/docs/enterprise` | Status pages if linked |
| Billing, account, charges | `/help/account-and-billing/*`, `/docs/models-and-pricing`, `/docs/account/teams/pricing` | Forum Account & Billing |
| UI behavior, copy/paste, panels, layout-specific shortcuts | Forum Help + Bug Reports + `/docs/agent/overview` | Staff replies, changelog |
| Keyboard shortcuts | search `site:cursor.com/docs shortcuts` (path moves) | Forum |
| Tab / autocomplete | `/docs/tab/overview` | Forum |
| Evals / programmatic agent scoring | `/docs/evals`, `/docs/sdk/typescript` | — |

Seed URLs for all of these: [sources.md](sources.md). Broad "everything about X" → Deep depth.

## Layout verification (UI questions only)

Cursor has shipped multiple chat/agent layouts over time. **Names, shortcuts, and feature parity between them change across versions** — never assert layout names, keybindings, or parity from memory (including any shortcut you "remember"). When the question concerns UI behavior, copy/paste, panels, layout-specific shortcuts, feature availability, or a bug tied to one layout:

1. Fetch current docs for the layout names and behavior as of today.
2. Fetch staff forum threads in full (badge verified) when docs are silent on a layout split.
3. State explicitly which layout each claim applies to, date-stamped, quoting the source's wording for shortcuts.
4. If you cannot tell which layout the user is in, ask or qualify both — do not assume.

Skip this entirely for pricing, billing, SDK methods, SSO, CLI flags, and other non-UI topics.

## Playbooks

**Pricing / plan gates** — always Deep. Fetch `/docs/models-and-pricing` and cross-check `cursor.com/pricing`; for Teams/Enterprise seats also fetch `/docs/account/teams/pricing` (marketing and docs pages have disagreed before). Check `/docs/enterprise` for plan-gated features and `/help/account-and-billing/*` + changelog for recent changes. Never single-source a pricing answer, and never copy numbers from sources.md or memory.

**Bug / missing feature** — search forum Bug Reports for the specific issue; check whether staff acknowledged it (quote exact wording + link); check changelog for fixes; surface workarounds from the thread labeled **community-reported** unless staff-confirmed. Distinguish "known issue, fix planned" from "unreported". Forum Ideas posts are feature requests — never present them as shipped.

**Privacy / security** — product docs alone are insufficient for policy-level claims. Also check the privacy policy, terms of service, trust/security pages, and official status pages where relevant. Quote exact wording for commitments.

**API / SDK / CLI** — fetch the actual reference page (TypeScript at `/docs/sdk/typescript`, Python at `/docs/sdk/python`, REST overview at `/docs/api`, Cloud Agents REST at `/docs/cloud-agent/api/endpoints`, CLI at `/docs/cli*`, evals at `/docs/evals`). Do not describe a method, flag, or endpoint from memory; the surface shifts across versions. Quote signatures verbatim.

## Output format (adaptive)

**Quick:**

```markdown
## Answer
[Direct answer — lead with the answer, not the research process. 1–2 inline citations.]
```

**Standard:**

```markdown
## Answer
[Direct answer in plain language.]

## Evidence
[Key findings, inline citations, confidence labels where helpful.]

## Unverified
[Specific gaps only — e.g. "whether X works in the agents layout as of 2026-06-14". Omit if none.]
```

**Deep:** Standard format plus:

```markdown
## Shared Fact Sheet
- **User question:** [one line]
- **Direct answer:** [concise]
- **Confidence:** [primary label]
- **Key details:** [facts with source links, date-stamped if volatile]
- **What works / what doesn't:** [confirmed behavior · gaps, bugs, limits]
- **Version / plan requirements:** [Cursor X.Y+; plan name, date-stamped] (if stated)
- **Staff status:** [name + badge verified + quote + URL] (if found)
- **Workaround:** [exact steps] (if found)
- **Related threads:** [URLs]

## Sources
[Numbered list of every URL fetched this session, with one-line description + fetch date]
```

## Output discipline

- Lead with the answer. Do not narrate the search process ("I searched…", "According to my research…").
- Match depth to the question: Quick never invents a Shared Fact Sheet; Deep never omits Sources.
- Shared Fact Sheet fields stay empty or omitted when unknown — never invent staff names, workaround steps, or version numbers to fill the template.
- Inline citations use URLs you fetched this session only.

## Chaining with humanizer

This skill produces research; voice is the humanizer skill's job. If the user wants the answer humanized or shaped as a forum reply, run this skill first, then invoke **humanizer** on the Answer (and Evidence if present). Hand off explicitly: preserve inline citations, confidence labels (`**Verified official**`, etc.), date stamps (`verified YYYY-MM-DD`), and any UNVERIFIED gaps — then confirm they survived the rewrite. Never auto-invoke humanizer; never strip labels unless the user asks.
