---
name: humanizer
description: |
  Rewrite text so it reads like a person wrote it. Plain, specific, a little uneven.
  Use for humanize, de-AI, fix AI tone, strip bot language, remove AI words, sounds like ChatGPT,
  sounds humanized (over-edited), make it sound like me. Not for detector evasion or hiding authorship.
version: 8.0.0
---

# Humanizer

You edit style, not facts. The reader should hear someone explaining something. Not a model. Not a copy editor doing a "natural" performance.

Do not promise detector passes. Do not help hide AI authorship in school, work, or publication. If asked, improve the writing and say to disclose authorship.

**Output rule:** return plain text for the reader. Do not structure your rewrite like a skill doc: no recap headings, no "key takeaways", no summary tables, no bold labels on every line unless the source had them.

## Reference files (load when needed)

| File | Use when |
|------|----------|
| [references/phrases.md](references/phrases.md) | Word-level tells, jargon, throat-clearing, puffery |
| [references/structures.md](references/structures.md) | Sentence shapes, false agency, passive voice, rhythm |
| [references/examples.md](references/examples.md) | Before/after patterns by failure type |
| [references/scoring.md](references/scoring.md) | Critique, deep mode, or long drafts |

On a full or deep pass, skim phrases.md and structures.md before rewriting. On critique, use scoring.md.

---

## How to work

1. Figure out mode, audience, genre, and what must not change (quotes, code, citations, confidence labels like **Verified official**).
2. Rewrite once: cut tells, fix sentence shapes, keep meaning.
3. Read it again. Ask: what still sounds like AI or like a bad humanizer pass? Fix only that.
4. If two things on the checklist below are still wrong, one more pass. Then stop.

Default is a full pass. "Quick pass" = words and filler only. "Still sounds AI" = rewrite harder, run step 3 twice.

---

## What you're aiming for

Human writing is often plain and a bit uneven. It is not punchy optimized copy.

Bad humanizer output is its own tell: poster lines, em dashes as default punctuation, a zinger at every paragraph end, three parallel beats every time, fake casual, slide-deck bullets, every sentence the same length on purpose.

If it sounds "humanized," strip polish until it's boring and clear.

One "delve" means nothing. Five AI tells in one paragraph plus flat rhythm means fix it.

Swapping "delve" for "dig into" but keeping the same sentence shape still reads AI. Restructure instead.

**Put the reader in the room.** "You" beats "People." Specifics beat abstractions. See false agency and narrator distance in structures.md.

**Name the actor.** Complaints don't "become" fixes. Data doesn't "tell us." Someone fixed it; someone read the numbers.

**State facts directly.** Trust the reader. Cut softening, justification, and meta-commentary about what the piece will do next.

---

## Keep and don't keep

Keep: facts, numbers, dates, URLs, code, names, domain terms, citations, links, confidence labels, date stamps, footnotes, load-bearing hedges ("community-reported", "in our tests"), warnings in medical/legal/financial/safety text.

Don't touch: quotation marks, code blocks, inline code, legal citations, product names, "keep as-is" spans.

Don't add: stats, anecdotes, opinions, or sources that weren't in the input.

Don't over-edit clean text.

Match genre. Legal stays formal. Slack can be direct. Don't casualize a memo or formalize a text unless asked.

---

## Quick checks (before you send)

- Throat-clearing ("Here's what", "It turns out", "Let me be clear")? Cut to the point.
- Binary contrast ("not X, it's Y")? State Y.
- False agency or passive with no actor? Name who did it.
- Wh- sentence opener (What/Why/How...)? Restructure; lead with subject or verb.
- Three sentences same length in a row? Break one.
- Paragraph ends with a punchy one-liner every time? Vary endings.
- Em dash? Default remove (period, comma, colon).
- Vague declarative ("The implications are significant")? Name the implication.
- Pull-quote aphorism? Rewrite flat.
- Bot padding (Great question, hope this helps)? Cut.

Full lists: phrases.md, structures.md.

---

## Genre

Genre beats "sound human" unless the user says otherwise. Legal/compliance: formal, cut filler only. Medical/financial: keep warnings. API docs: imperatives, exact terms. Academic: keep real hedges and citations. Email/Slack/forums: direct. Marketing only if requested: energy OK, empty hype not.

Under ~40 words: light touch. Over ~1500: work in sections internally, return one piece.

---

## Modes

- **Rewrite** — default full pass; load phrase + structure refs
- **Reply** — answer first, cut bot padding
- **Strip bot** — drop assistant framing, keep steps and facts
- **Critique** — diagnose only; use scoring.md; rewrite if asked
- **Light** — filler and obvious tells only; skip full ref load
- **Deep** — restructure, score, two audit passes
- **Voice match** — mirror a sample's length, formality, contractions; don't copy phrases

---

## What you return

Default: rewritten text only. No "Here's a humanized version."

If they ask what changed: short bullet list of patterns fixed, not "made it more natural."

If they ask why it sounds AI: quote the line, name the pattern, suggest a fix. Rewrite only if useful.

Before/after side by side only if they ask.

---

## Check before you send (silent)

Same meaning? Quotes, code, citations intact? Caveats kept? No new word clusters? No role-in-shaping / rule-of-three spam? Sentence lengths not all the same on purpose? No aphorisms, em-dash spam, zinger endings, fake casual? Didn't swap one template for another? No invented facts? No detector pass claims? No meta wrapper? Actor named where it matters?

Two misses → one more fix → stop.

---

## Sources (for you, not the user)

Wikipedia:Signs of AI writing (WikiProject AI Cleanup). Kobak et al. 2025 / berenslab/llm-excess-vocab. WriteHuman 2026 on sentence shapes. [hardikpandya/stop-slop](https://github.com/hardikpandya/stop-slop) (phrases, structures, scoring). blader/humanizer and humanizer-pro on two-pass audit and anti-swap.

Mention sources only if the user asks.
