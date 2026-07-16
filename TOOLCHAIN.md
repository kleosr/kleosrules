# TOOLCHAIN — Documentos/rules (SSOT harness)

Done = these green (evidence in the report). Do not invent extras.

For any shell-script edit:

```bash
bash -n /home/kleosr/Documentos/rules/sync-to-repos.sh \
  /home/kleosr/Documentos/rules/verify-sync.sh \
  /home/kleosr/Documentos/rules/scan-and-sync.sh \
  /home/kleosr/Documentos/rules/lib/discover-repos.sh \
  /home/kleosr/Documentos/rules/hooks/block-dangerous-git.sh \
  /home/kleosr/Documentos/rules/hooks/install-user-hooks.sh
command -v shellcheck >/dev/null && shellcheck \
  /home/kleosr/Documentos/rules/sync-to-repos.sh \
  /home/kleosr/Documentos/rules/verify-sync.sh \
  /home/kleosr/Documentos/rules/scan-and-sync.sh \
  /home/kleosr/Documentos/rules/lib/discover-repos.sh \
  /home/kleosr/Documentos/rules/hooks/block-dangerous-git.sh \
  /home/kleosr/Documentos/rules/hooks/install-user-hooks.sh || true
```

After editing any root `*.mdc`, managed Skill, scan config, or manifest:

```bash
bash /home/kleosr/Documentos/rules/scan-and-sync.sh
```

Or stepwise:

```bash
bash /home/kleosr/Documentos/rules/sync-to-repos.sh
bash /home/kleosr/Documentos/rules/verify-sync.sh
```

List discovery only:

```bash
bash /home/kleosr/Documentos/rules/lib/discover-repos.sh
```

User Rules presence (best-effort, non-fatal in verify):

```bash
python3 /home/kleosr/Documentos/rules/lib/check-user-rules.py; echo exit:$?
```

Discovery SSOT: `scan.roots` + `scan.ignore`. Managed Skills: `skills.txt`.
Retired artifacts: `retired.txt`, `retired-skills.txt`.

Periodic rescan: run `scan-and-sync.sh` on a schedule (e.g. daily) or after
cloning a new project under a scan root.
