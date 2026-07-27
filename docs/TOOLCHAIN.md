# TOOLCHAIN — this pack (Master Mind V15.1)

```bash
PACK="$(cd "$(dirname "$0")/.." && pwd)"
bash -n "$PACK"/install.sh \
  "$PACK"/scripts/*.sh \
  "$PACK"/lib/discover-repos.sh \
  "$PACK"/hooks/*.sh
(cd "$PACK"/hooks/kleos-gate && cargo test && cargo build --release)
mkdir -p "$PACK"/hooks/bin
cp -f "$PACK"/hooks/kleos-gate/target/release/kleos-gate "$PACK"/hooks/bin/kleos-gate
chmod +x "$PACK"/hooks/bin/kleos-gate
FORCE_SKILLS=1 bash "$PACK"/install.sh
bash "$PACK"/scripts/sync-hooks-to-repos.sh
bash "$PACK"/scripts/verify-sync.sh
```

After editing `project-rules/*.mdc` or `config/*`:

```bash
bash scripts/scan-and-sync.sh
```
