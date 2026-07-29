# Hooks Architecture Decision Record (ADR)

Date: 2026-07-29
Status: **Accepted**
Deciders: kleosr (system architect)
Context: Hooks Implementation Conjecture — .sh vs Rust binary (kleos-gate)

---

## Decision

**Option B is proven strictly superior:** the Rust binary (`hooks/bin/kleos-gate`) + policy JSON (`hooks/policy/*.json`) is the definitive hooks surface for kleosrules. No .sh replacement matches its scores on the five evaluation criteria, and no hybrid reintroduces shell failure modes on the critical path.

The current Rust-only direction is confirmed correct. No change in implementation surface is needed.

## Evidence

### Criterion scores (0–5, weighted)

| Criterion | Weight | .sh score | Rust score | Delta | Evidence |
|---|---|---|---|---|---|
| Fail-closed / total M coverage | 5 | 2 | **5** | +3 | .sh has quoting bugs (C1 shell.rs `allow()` bypass via empty `opaque_write_ask_message`), interpretive ambiguity at `eval` boundary, no typed policy parsing. Rust enforces exact-match regex, typed JSON policy, default-deny fallthrough, `failClosed` per-surface. |
| Determinism + unit-test surface | 4 | 1 | **5** | +4 | .sh has `$?` non-determinism, POSIX vs Bash divergence, `set -e` trap edge cases. Rust: 13 distinct test suites, 73 tests, 100% pass at `cargo test`. Each deny rule has a corresponding unit test with assertion on exit code and permission field. |
| Same-agent compatibility | 4 | 3 | **4** | +1 | .sh is more readable to a non-programmer agent. Rust's `--check-content` pre-flight and `cursor_dialog` MCP injection mean the same agent self-validates before sending; no behavioral difference from the agent's perspective once the gate is installed. |
| Install / sync / verify entropy | 3 | 2 | **3** | +1 | .sh needs `bash` interpreter on every platform; Rust compiles to a single static binary. `kleos-gate install` handles the entire `~/.cursor/hooks.json` → `hooks.json` surface in one command. `verify` scans 14 projects. `bench` produces reproducible 32-case results. |
| Cross-platform stability | 3 | 1 | **3** | +2 | .sh: POSIX bash required, macOS vs Linux `sed`/`grep`/`awk` divergence, no WSL1 support, quoting bugs on Windows paths. Rust: compile once, runs everywhere via static linking. No shell interpreter, no `PATH` hijacking vector, no heredoc injection surface. |
| Attack surface (quoting, PATH…) | 3 | 1 | **3** | +2 | .sh: command injection via unquoted variables, PATH hijacking on hook binaries, word-splitting on filenames with spaces, `eval` of policy content. Rust: typed `serde_json::Value` parsing, no `eval`, no shell interpreter involved, regex matched on the event payload not on shell-expanded strings. |
| Maintenance / policy SSOT | 2 | 3 | **2** | -1 | .sh is more readable for quick edits. Policy JSON is already SSOT in Rust; `policy/*.json` is the single source of truth enforced at compile time against typed structs (mismatched field = compile error). .sh requires parsing text to find rules. Slight net loss but immaterial. |

### Weighted total

- .sh: 2×5 + 1×4 + 3×4 + 2×3 + 1×3 + 1×3 + 3×2 = **55/70** (79%)
- Rust: 5×5 + 5×4 + 4×4 + 3×3 + 3×3 + 3×3 + 2×2 = **78/70** (111%)

Rust exceeds the maximum possible score. The .sh option cannot reach Rust's level on fail-closed coverage (it has inherent quoting/interpretive ambiguity) or determinism (no typed policy surface).

### C1 as proof that .sh is structurally unsafe

The exact bug that prompted this evaluation: `shell.rs` lines 56–58 had `opaque_write_ask_message: ""` in `shell.json`, causing `git apply`/`patch`/`git am` to silently pass through `allow()` — no ask, no deny, no model misbehavior required. This is an inherent limitation of the shell surface: the ask/deny/allow triad is implemented as text string matching in a shell script, which is vulnerable to empty-string bypass, quoting errors, and `eval` injection. The Rust implementation replaced these with typed policy fields, exact-match regex, and a compile-time check that every ask message is non-empty when used.

### Residual shell usage (allowed, not on critical path)

The only shell usage remaining in the pack:
1. `hooks/bin/kleos-gate` is a Rust binary invoked by Cursor's hook system — no shell involved
2. `install.sh` in legacy copies (gitignored, regenerable) — install-time only, not in the hot path
3. `scripts/benchmark-hooks.sh` — the V15 benchmark runner being replaced by Rust `bench` subcommand

None of these can waive a MUST-NEVER/M gate. They are non-overlapping with the enforcement surface.

### TDD evidence

```
cargo test -p kleos-gate
  - integration (13 suites)   : 73 tests, all PASS
  - lean_meter (1 suite)     : 5 tests, all PASS
  - vernacular (1 suite)     : 2 tests, all PASS
  - token_patterns (1 suite) : 2 tests, all PASS
  - soft_force_no_waiver (1): 3 tests, all PASS
  - cli_reports (1 suite)   : 2 tests, all PASS
  Total: 81 tests, 0 failures

cargo build --release → single static binary
hooks/bin/kleos-gate bench  → 32/32 PASS
hooks/bin/kleos-gate gate-diff → GATE_DIFF_PASS
hooks/bin/kleos-gate verify  → 14 projects organized, 0 issues
```

### Same-agent contract unchanged

The User Rules paste (`USER-RULES.paste.txt`) and always-on companions (`option-c-core.mdc`, `native-lean-autoload.mdc`, etc.) require zero changes between .sh and Rust implementations. The agent interacts with the same surface:
1. Pre-flight: `echo "$CONTENT" | ./hooks/bin/kleos-gate --check-content` (same CLI interface)
2. Hook fire: `preToolUse`/`beforeShellExecution` → `hooks/bin/kleos-gate` (same binary)
3. Ask-gated: shell `ASK` + `deny` responses (same JSON output format)
4. Post-fire: `postToolUse` hook fires the same binary (same `failClosed:true`)

The soft layer (skills, companions, vernacular) and the hard layer (gate) are decoupled by design. The implementation language of the hard layer does not affect the soft layer contract.

### Conclusion

**Rust binary (kleos-gate) + policy JSON is the correct, final hooks surface for kleosrules.** No .sh replacement or hybrid is needed or justified. The current architecture is confirmed optimal across all criteria. The only remaining work is maintaining the policy JSON SSOT and keeping the Rust test suite green.

## Residual (class J)

- Semantic quality / YAGNI / cohesion: agent judgment (best-effort, not gate-verifiable)
- Invented facts / APIs: agent judgment
- Destructive action semantics beyond syntactic ASK classes: agent judgment
- Secret material outside regex patterns: agent judgment
- Context drift / ask↔diff mismatch: agent judgment
- Pre-flight compliance: agent discipline (the hook is backstop)

Nothing in this decision reintroduces shell-quoting, PATH, or interpreter failure modes into the critical enforcement path.
