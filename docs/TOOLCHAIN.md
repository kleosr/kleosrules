# TOOLCHAIN — this pack (Master Mind V16.0.20)

```bash
PACK="$(cd "$(dirname "$0")/.." && pwd)"
(cd "$PACK"/hooks/kleos-gate && cargo test && cargo build --release)
mkdir -p "$PACK"/hooks/bin
cp -f "$PACK"/hooks/kleos-gate/target/release/kleos-gate "$PACK"/hooks/bin/kleos-gate
chmod +x "$PACK"/hooks/bin/kleos-gate
FORCE_SKILLS=1 "$PACK"/hooks/bin/kleos-gate install
"$PACK"/hooks/bin/kleos-gate sync-hooks
"$PACK"/hooks/bin/kleos-gate verify
"$PACK"/hooks/bin/kleos-gate bench
"$PACK"/hooks/bin/kleos-gate gate-diff
"$PACK"/hooks/bin/kleos-gate check-user-rules
```

After editing `project-rules/*.mdc` or `config/*`:

```bash
hooks/bin/kleos-gate sync
```
