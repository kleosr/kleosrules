#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

FIELD_RE = re.compile(
    r"^(file_name_pattern|allowed_kinds|class_pattern|function_pattern|"
    r"boolean_prefixes|constant_pattern|no_prose_comments|machine_directives_only)\s*:\s*(.+)$",
    re.M | re.I,
)
CLASS_RE = re.compile(r"(?:^|[^A-Za-z0-9_-])class\s+([A-Z][A-Za-z0-9_]*)")
FUNC_RE = re.compile(
    r"(?:^|\n)\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)"
    r"|(?:^|\n)\s*def\s+([A-Za-z_][A-Za-z0-9_]*)"
    r"|(?:^|\n)\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_][A-Za-z0-9_]*)\s*="
)
CONST_RE = re.compile(
    r"(?:^|\n)\s*(?:export\s+)?(?:const|let|var)\s+([A-Z][A-Z0-9_]{2,})\s*="
)

PASCAL = re.compile(r"^[A-Z][A-Za-z0-9]*$")
SNAKE = re.compile(r"^_?[a-z][a-z0-9_]*$")
CAMEL = re.compile(r"^[a-z][A-Za-z0-9]*$")
SCREAM = re.compile(r"^[A-Z][A-Z0-9_]*$")
DOMAIN_KIND_EXT = re.compile(
    r"^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*\.[a-z][a-z0-9_.]*$"
)


def out(d):
    print(json.dumps(d))
    raise SystemExit(0)


def find_contract(start: Path) -> Path | None:
    cur = start if start.is_dir() else start.parent
    for _ in range(32):
        for name in (
            cur / ".cursor" / "rules" / "vernacular.mdc",
            cur / "VERNACULAR.md",
            cur / "docs" / "VERNACULAR.md",
        ):
            if name.is_file():
                return name
        if cur.parent == cur:
            break
        cur = cur.parent
    return None


def parse_fields(text: str) -> dict:
    fields = {}
    for m in FIELD_RE.finditer(text):
        key = m.group(1).lower()
        val = m.group(2).strip().strip("`").strip()
        if val.upper() == "TBD" or not val:
            continue
        if key == "allowed_kinds":
            if val in ("[]", "none", "-"):
                fields[key] = []
            else:
                fields[key] = [x.strip() for x in re.split(r"[, ]+", val) if x.strip()]
        else:
            fields[key] = val
    return fields


def file_name_ok(name: str, fields: dict) -> bool:
    pat = fields.get("file_name_pattern", "")
    if not pat or pat in ("kebab_or_snake", "pack_native", "free"):
        return True
    if pat == "domain.kind.ext":
        parts = name.split(".")
        if len(parts) < 3:
            return False
        if not all(re.match(r"^[a-z][a-z0-9_]*$", p) for p in parts):
            return False
        kinds = fields.get("allowed_kinds") or []
        if kinds:
            kind = parts[-2]
            return kind in kinds
        return True
    return True


def naming_ok(text: str, fields: dict) -> tuple[bool, str]:
    cp = fields.get("class_pattern", "")
    if cp.startswith("PascalCase"):
        for m in CLASS_RE.finditer(text):
            if not PASCAL.match(m.group(1)):
                return False, f"class {m.group(1)} violates {cp}"

    fp = fields.get("function_pattern", "")
    if fp in ("snake_case", "verbObject"):
        for m in FUNC_RE.finditer(text):
            name = next(g for g in m.groups() if g)
            if name.isupper() and "_" in name:
                continue
            if fp == "snake_case" and not SNAKE.match(name):
                if SCREAM.match(name):
                    continue
                return False, f"name {name} violates snake_case"
            if fp == "verbObject" and not CAMEL.match(name):
                if SCREAM.match(name):
                    continue
                return False, f"name {name} violates verbObject"

    kp = fields.get("constant_pattern", "")
    if kp == "SCREAMING_SNAKE_CASE":
        for m in CONST_RE.finditer(text):
            if not SCREAM.match(m.group(1)):
                return False, f"const {m.group(1)} violates SCREAMING_SNAKE_CASE"
    return True, ""


def fail_deny():
    out({
        "permission": "deny",
        "user_message": "Vernacular gate error (fail-closed).",
        "agent_message": "deny-vernacular-drift failed closed. Fix the hook and retry.",
    })


def main() -> None:
    try:
        data = json.load(sys.stdin)
    except Exception:
        fail_deny()

    inp = data.get("input") or data.get("tool_input") or {}
    if not isinstance(inp, dict):
        inp = {}

    path = str(inp.get("path") or data.get("path") or data.get("file_path") or "")
    if not path:
        out({"permission": "allow"})

    p = Path(path)
    contract = find_contract(p if p.exists() else p.parent)
    if contract is None:
        out({"permission": "allow"})

    fields = parse_fields(contract.read_text(encoding="utf-8", errors="replace"))
    if not fields:
        out({"permission": "allow"})

    base = p.name
    if not file_name_ok(base, fields):
        out({
            "permission": "deny",
            "user_message": f"Blocked vernacular file-name drift: {base}",
            "agent_message": (
                f"File name {base} violates vernacular at {contract}. "
                f"Expected file_name_pattern={fields.get('file_name_pattern')}."
            ),
        })

    try:
        from hookio import walk_strings
        chunks = walk_strings(inp) + walk_strings(
            {k: data[k] for k in data if k not in ("tool_input", "input")}
        )
    except Exception:
        chunks = []
        for k in ("contents", "new_string", "content", "cell_content", "new_source"):
            v = inp.get(k)
            if isinstance(v, str):
                chunks.append(v)
    text = "\n".join(chunks)
    if text:
        ok, reason = naming_ok(text, fields)
        if not ok:
            out({
                "permission": "deny",
                "user_message": f"Blocked vernacular naming drift: {reason}",
                "agent_message": (
                    f"{reason}. Follow vernacular at {contract}."
                ),
            })

    out({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        fail_deny()
