# Humanizer → Wispr Flow

Wispr Flow does not detect AI authorship. It rewrites selected text via **Transforms** and shows edits in **View Diff** (Opt+O / Win+Alt+O).

Use this folder to copy prompts into Flow: Transforms tab → Create your own.

## Setup (about 5 minutes)

1. **Transform slot 2 — Audit** (optional): paste `audit-prompt.txt`. Shortcut e.g. Opt+3. Select AI text → run → get a tell list, no rewrite.
2. **Transform slot 3 — De-AI**: paste `de-ai-prompt.txt`. Shortcut e.g. Opt+4. Select text → run → humanized rewrite.
3. **Polish card**: add all five lines from `polish-instructions.txt` (50 words max each).
4. **Writing samples**: on the De-AI transform, add the five blocks from `writing-samples.txt` (each 50–500 words).
5. **View Diff**: after De-AI, Opt+O to see every change Flow made.

## Workflow

- Pasted ChatGPT text: Audit → De-AI → View Diff.
- Your dictation: set Auto Apply After Dictation to **De-AI** only if you ramble; otherwise Polish is enough.
- 1000-word limit per transform. Split long docs.

## What Flow can mirror from the skill

| Skill behavior | Flow mechanism |
|----------------|----------------|
| Rewrite plain | De-AI custom transform |
| Critique / list tells | Audit custom transform |
| Voice match | Writing samples on De-AI prompt |
| Light pass | Polish + custom instructions |
| See what changed | View Diff |

Custom transform slots do not sync across devices; re-paste on each Mac/Windows machine.
