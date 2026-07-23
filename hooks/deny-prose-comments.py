#!/usr/bin/env python3
"""Deny prose comments in application-code Write/StrReplace/EditNotebook payloads."""
import json, re, sys

CODE_EXT = (
    ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs",
    ".py", ".go", ".rs", ".java", ".kt", ".swift",
    ".rb", ".php", ".cs", ".cpp", ".cc", ".c", ".h", ".hpp",
)
SKIP_EXT = (
    ".sh", ".bash", ".zsh", ".fish", ".ps1",
    ".md", ".mdc", ".txt", ".json", ".yml", ".yaml", ".toml",
    ".lock", ".svg", ".html", ".css", ".scss",
)

JS_OK = re.compile(
    r"^\s*//\s*("
    r"@ts-(expect-error|ignore|nocheck|check)|"
    r"eslint-disable(?:-next-line)?|"
    r"prettier-ignore|"
    r"istanbul ignore|"
    r"biome-ignore|"
    r"v8 ignore"
    r")\b",
    re.I,
)
JS_PROSE = re.compile(r"^\s*//(?!\s*$).+")
PY_OK = re.compile(
    r"^\s*#\s*(type:\s*ignore|noqa|pragma:|pylint:|mypy:|fmt:|ruff:)",
    re.I,
)
PY_SHEBANG = re.compile(r"^#!")
PY_CODING = re.compile(r"^\s*#.*coding[:=]", re.I)
PY_PROSE = re.compile(r"^\s*#(?!!).+")


def out(d):
    print(json.dumps(d))
    raise SystemExit(0)


def path_is_code(path: str) -> bool:
    p = (path or "").split("?")[0].lower()
    if not p:
        return False
    if any(p.endswith(ext) for ext in SKIP_EXT):
        return False
    return any(p.endswith(ext) for ext in CODE_EXT)


def has_prose_js(text: str) -> bool:
    if re.search(r"/\*", text):
        for m in re.finditer(r"/\*.*?\*/", text, re.S):
            body = m.group(0)
            if re.match(
                r"^/\*\s*(eslint|prettier|istanbul|biome|ts-|@ts-)",
                body,
                re.I,
            ):
                continue
            if re.match(r"^/\*\s*\*/$", body):
                continue
            return True
        if re.search(r"/\*(?![\s\S]*?\*/)", text):
            return True
    for line in text.splitlines():
        if JS_PROSE.match(line) and not JS_OK.match(line):
            return True
    return False


def has_prose_py(text: str) -> bool:
    for i, line in enumerate(text.splitlines()):
        if i == 0 and PY_SHEBANG.match(line):
            continue
        if PY_CODING.match(line) or PY_OK.match(line):
            continue
        if PY_PROSE.match(line):
            return True
    return False


def violates(path: str, text: str) -> bool:
    if not text or not path_is_code(path):
        return False
    if path.lower().endswith(".py"):
        return has_prose_py(text)
    return has_prose_js(text)


try:
    data = json.load(sys.stdin)
except Exception:
    out({"permission": "allow"})

inp = data.get("input") or data.get("tool_input") or {}
if not isinstance(inp, dict):
    inp = {}

path = str(inp.get("path") or data.get("path") or "")
chunks = []
for k in ("contents", "new_string", "content", "cell_content", "new_source"):
    v = inp.get(k)
    if isinstance(v, str):
        chunks.append(v)
    v2 = data.get(k)
    if isinstance(v2, str):
        chunks.append(v2)

text = "\n".join(chunks)
if violates(path, text):
    out({
        "permission": "deny",
        "user_message": "Blocked prose comment in code write (Native Lean NO COMMENTS).",
        "agent_message": (
            "Blocked prose comment. Never write // or # or /* */ prose in app code. "
            "Use names/structure; machine directives only if required for green build."
        ),
    })
out({"permission": "allow"})
