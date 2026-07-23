---
name: benln-write
description: >-
  Writes short, human cold emails and replies in Ben Lang (@benln) style: 2–3
  sentences, clear ask, casual, no fluff. Use when the user asks for Ben Lang /
  benln / cold email style, outreach replies, short ask-heavy messages, ProtonMail
  drafts, "write like the Ben thread", vouch/trial negotiation replies, or
  status-check messages that should read like a text.
---

# Benln Write

Write like Ben Lang’s cold email posts: short, specific, one clear ask, sounds like a text — not a sales letter or AI.

Sources of truth:
- Checklist: https://x.com/benln/status/2037256558791307276
- Full guide: https://x.com/benln/status/2018035287851704556
- Essay: https://nextplayso.substack.com/p/the-guide-to-getting-a-job-with-cold

## Output rule

Return the draft ready to send (markdown or plain text). No recap headings, no "here's why this works", no alternatives unless the user asks. If they want ProtonMail / markdown, wrap the body in a markdown code fence.

Default voice: lowercase, casual, plain English. Match the user’s name/sign-off if they give one; otherwise omit signature (Ben: no signature / too formal). If they include a sign-off (e.g. `best, Mario`), keep it.

## Workflow

1. Extract: who the recipient is, what’s specific/true, what value or leverage exists, the **one** ask.
2. Structure (from the guide):
   - Who you are
   - Why you’re writing
   - Why they should care
   - One clear ask
3. Cut until it’s ~2–4 short sentences (hard ceiling ~200 words; prefer under 80).
4. Self-check against the rules below, then output only the draft.

## Hard rules

Do:
- 2–3 sentences that read like a text (ok to split across short lines)
- One explicit ask that’s easy to answer (`deal?`, `can you do that?`, `any status?`)
- Real personalization (repo name, prior email, real credential) — never fake “saw your LinkedIn”
- Plain words; read it out loud — must sound like a person
- Specifics over abstractions (`cursorkleosr`, `vouch + free trial`, email addresses)

Don’t:
- Fancy / buzzword language (`synergy`, `leverage`, `circle back`, `hope this finds you well`)
- Vague asks (`pick your brain`, `hop on a quick call sometime`, `let me know your thoughts`)
- Long backstory, feature lists, or multi-ask menus
- Formal signature blocks unless the user wants one
- AI tells: em dashes everywhere, triple adjectives, “I’d love to”, “excited to connect”
- Lie or invent status/credentials

## Modes

| Mode | When | Shape |
|------|------|--------|
| Cold / first touch | New outreach | who → why → why care → ask |
| Reply | Answering inbound (sales, outreach) | ack the note → leverage → counter-ask |
| Internal ping | Status / teammate check | name + fact + one question |
| ProtonMail | User says proton / markdown | same body in a ```markdown fence |

## Quick templates

**Reply (inbound pitch):**
```markdown
hey — saw the note on [specific thing].

i'm [who you are] with [relevant proof].
open to [their offer] if we meet in the middle: [your ask].

deal?
```

**Internal status check:**
```markdown
hey [name] — quick check on [person/email].

[what they claimed]. [what you already told them].
any status on [application / thing]?
```

## Examples

See [examples.md](examples.md).

## Done when

- One ask only
- Specific names/facts present
- Sounds human when read aloud
- Under ~80 words unless the user asked for more
