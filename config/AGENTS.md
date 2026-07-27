# AGENTS.md — map (nested)

**Parent map:** [`../AGENTS.md`](../AGENTS.md)

## Scope

Install/sync inputs: which skills ship, which repos get scanned, retire lists.

## Where to look

| File | Role |
|------|------|
| `skills.txt` | Harness-owned personal skill directory names |
| `scan.roots` | Fleet roots for `scan-and-sync.sh` |
| `scan.ignore` | Paths/repos skipped by scan |
| `retired-skills.txt` | Skills not to install |
| `retired.txt` | Retired pack items |
| `repos.txt.deprecated` | Legacy — do not use |

## Done (local)

After edits: `bash scripts/scan-and-sync.sh` then `bash scripts/verify-sync.sh`.

## Ask first

- Adding scan roots that write into many live app repos

## Manual notes

<!-- Preserved on refresh -->
