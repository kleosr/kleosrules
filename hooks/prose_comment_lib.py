#!/usr/bin/env python3
from __future__ import annotations

import base64
import io
import re
import tokenize
from typing import List, Tuple

NON_CODE_EXT = (
    ".md", ".mdc", ".txt", ".json", ".yml", ".yaml", ".toml", ".lock",
    ".svg", ".png", ".jpg", ".jpeg", ".gif", ".webp", ".ico",
    ".html", ".htm", ".css", ".scss", ".sass", ".less",
    ".csv", ".tsv", ".xml", ".pdf", ".zip", ".gz", ".tgz", ".xz",
    ".woff", ".woff2", ".ttf", ".eot",
    ".map", ".min.js", ".min.css",
)
SHELL_EXT = (".sh", ".bash", ".zsh", ".fish", ".ps1")

JS_DIR_OK = re.compile(
    r"^\s*(?:"
    r"@ts-(?:expect-error|ignore|nocheck|check)|"
    r"eslint-disable(?:-next-line)?|"
    r"prettier-ignore|"
    r"istanbul ignore|"
    r"biome-ignore|"
    r"v8 ignore"
    r")\b",
    re.I,
)
PY_DIR_OK = re.compile(
    r"^\s*(?:type:\s*ignore|noqa|pragma:|pylint:|mypy:|fmt:|ruff:|"
    r"tfsec:ignore|rubocop:disable|rubocop:todo)\b",
    re.I,
)
HASH_DIR_OK = PY_DIR_OK
DASH_DIR_OK = re.compile(r"^\s*(?:noqa|type:\s*ignore|tfsec:ignore)\b", re.I)

DENY_USER = "Blocked prose comment in code write (Native Lean NO COMMENTS)."
DENY_AGENT = (
    "Blocked prose comment. Never write prose comments in app code. "
    "Use names/structure; machine directives only if required for green build. "
    "Do not use Shell to write app code with comments — use Write/StrReplace only."
)

_SL = "/" + "/"
_BL = "/" + "*"
SHELL_JS_PROSE = re.compile(
    r"(?<!:)" + re.escape(_SL) + r"(?!\s*(?:@ts-|eslint|prettier|istanbul|biome|v8)\b)"
)
SHELL_PY_PROSE = re.compile(
    r"#" + r"(?!\s*(?:type:\s*ignore|noqa|pragma:|pylint:|mypy:|fmt:|ruff:|!|/|tfsec:ignore|rubocop:))"
)
SHELL_BLOCK = re.compile(re.escape(_BL))

CODEISH = re.compile(
    r"\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts|vue|svelte|py|go|rs|java|kt|swift|"
    r"rb|php|cs|cpp|cc|c|h|hpp|sql|dart|scala|sol|astro|lua|zig|tf|ex|exs|"
    r"r|jl|pl|clj|cljs|erl|hs|ml|mli|nim|cr|v|sv|vhd)\b",
    re.I,
)
REDIR_TO_CODE = re.compile(
    rf"(?:>>?|tee(?:\s+-a)?)\s*[\"']?[^\s\"';]+{CODEISH.pattern}",
    re.I,
)
HEREDOC_TO_CODE = re.compile(
    rf"cat\s*>+\s*[\"']?[^\s\"';]+{CODEISH.pattern}",
    re.I,
)
EVAL_WRITE = re.compile(
    rf"(?:python3?|node|deno)\s+-c\b.*(?:"
    rf"open\s*\(|write_text\s*\(|write_bytes\s*\(|Path\s*\([^)]*\)\s*\.\s*write_text|\.write\s*\()"
    rf".*{CODEISH.pattern}",
    re.I | re.S,
)
OPAQUE_WRITE = re.compile(
    r"(?:"
    r"\bsed\s+-i\b|"
    r"\bsed\s+[^\n]*?\s+-i\b|"
    r"\bgit\s+apply\b|"
    r"\bpatch\b|"
    r"\bapplypatch\b|"
    r"(?:python3?|node|deno|ruby|perl)\s+[^\n|>]+\s*>\s*[^\s]+|"
    r"base64\s+[^\n]*\b-d\b[^\n]*>>?\s*[^\s]+|"
    r"openssl\s+[^\n]+>>?\s*[^\s]+"
    r")",
    re.I,
)
BASE64_PIPE = re.compile(
    r"(?:echo|printf)\s+[^\n|]*\|\s*base64\s+[^\n]*-d[^\n]*(>>?)\s*([^\s]+)",
    re.I,
)
B64_TOKEN = re.compile(r"(?:echo|printf)\s+['\"]?([A-Za-z0-9+/=]{8,})['\"]?")


def path_is_code(path: str) -> bool:
    p = (path or "").split("?")[0].lower().rstrip("/")
    if not p:
        return True
    base = p.rsplit("/", 1)[-1]
    if "." not in base or base.startswith("."):
        return False
    if any(p.endswith(ext) for ext in NON_CODE_EXT):
        return False
    if any(p.endswith(ext) for ext in SHELL_EXT):
        return False
    return True


def _py_comment_nodes(text: str) -> List[str]:
    out: List[str] = []
    try:
        tokens = tokenize.generate_tokens(io.StringIO(text).readline)
        for tok in tokens:
            if tok.type != tokenize.COMMENT:
                continue
            body = tok.string
            if body.startswith("#!") and tok.start[0] == 1:
                continue
            inner = body[1:].lstrip()
            if re.match(r"coding[:=]", inner, re.I):
                continue
            if PY_DIR_OK.match(inner):
                continue
            if inner.strip():
                out.append(body)
    except tokenize.TokenError:
        for i, line in enumerate(text.splitlines()):
            s = line.lstrip()
            if not s.startswith("#"):
                continue
            if i == 0 and s.startswith("#!"):
                continue
            inner = s[1:].lstrip()
            if re.match(r"coding[:=]", inner, re.I) or PY_DIR_OK.match(inner):
                continue
            if inner.strip():
                out.append(s)
    return out


def _looks_like_regex(text: str, i: int) -> bool:
    j = i - 1
    while j >= 0 and text[j] in " \t":
        j -= 1
    if j < 0:
        return True
    return text[j] in "=(:,[!&|?+-{;\n"


def _js_comment_spans(text: str) -> List[Tuple[int, int, str]]:
    spans: List[Tuple[int, int, str]] = []
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        nxt = text[i + 1] if i + 1 < n else ""

        if c in "\"'":
            quote = c
            i += 1
            while i < n:
                if text[i] == "\\":
                    i += 2
                    continue
                if text[i] == quote:
                    i += 1
                    break
                i += 1
            continue

        if c == "`":
            i += 1
            while i < n:
                if text[i] == "\\":
                    i += 2
                    continue
                if text[i] == "`":
                    i += 1
                    break
                if text[i] == "$" and i + 1 < n and text[i + 1] == "{":
                    i += 2
                    depth = 1
                    while i < n and depth:
                        if text[i] == "{":
                            depth += 1
                        elif text[i] == "}":
                            depth -= 1
                        i += 1
                    continue
                i += 1
            continue

        if c == "/" and nxt == "/":
            start = i
            i += 2
            while i < n and text[i] not in "\r\n":
                i += 1
            spans.append((start, i, text[start:i]))
            continue

        if c == "/" and nxt == "*":
            start = i
            i += 2
            while i + 1 < n and not (text[i] == "*" and text[i + 1] == "/"):
                i += 1
            i = min(i + 2, n)
            spans.append((start, i, text[start:i]))
            continue

        if c == "/" and _looks_like_regex(text, i):
            i += 1
            while i < n:
                if text[i] == "\\":
                    i += 2
                    continue
                if text[i] == "/":
                    i += 1
                    while i < n and text[i].isalpha():
                        i += 1
                    break
                if text[i] in "\r\n":
                    break
                i += 1
            continue

        i += 1
    return spans


def _js_comment_nodes(text: str) -> List[str]:
    return [node for _, _, node in _js_comment_spans(text)]


def _js_prose_from_nodes(nodes: List[str]) -> bool:
    for node in nodes:
        if node.startswith(_BL):
            inner = node[2:]
            if inner.endswith("*/"):
                inner = inner[:-2]
            if BLOCK_DIR_OK_MATCH(inner):
                continue
            if inner.strip():
                return True
            continue
        if node.startswith(_SL):
            inner = node[2:]
            if JS_DIR_OK.match(inner):
                continue
            if inner.strip():
                return True
    return False


def BLOCK_DIR_OK_MATCH(inner: str) -> bool:
    return bool(re.match(r"^\s*(?:eslint|prettier|istanbul|biome|ts-|@ts-)", inner, re.I))


def _line_family_prose(text: str, marker: str, dir_ok: re.Pattern[str]) -> bool:
    in_s = in_d = in_t = False
    esc = False
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        if in_s or in_d or in_t:
            if esc:
                esc = False
                i += 1
                continue
            if c == "\\":
                esc = True
                i += 1
                continue
            if in_s and c == "'":
                in_s = False
            elif in_d and c == '"':
                in_d = False
            elif in_t and c == "`":
                in_t = False
            i += 1
            continue
        if c == "'":
            in_s = True
            i += 1
            continue
        if c == '"':
            in_d = True
            i += 1
            continue
        if c == "`":
            in_t = True
            i += 1
            continue
        if text.startswith(marker, i):
            if marker == "#" and i + 1 < n and text[i + 1] == "{":
                i += 1
                continue
            if marker == "-" and not text.startswith("--", i):
                i += 1
                continue
            rest = text[i + len(marker) :].splitlines()[0]
            if dir_ok.match(rest):
                i += len(marker) + len(rest)
                continue
            if rest.strip():
                return True
            i += len(marker) + len(rest)
            continue
        i += 1
    return False


def has_prose_py(text: str) -> bool:
    return bool(_py_comment_nodes(text))


def has_prose_js(text: str) -> bool:
    return _js_prose_from_nodes(_js_comment_nodes(text))


def text_has_prose(text: str, path: str = "") -> bool:
    if not text:
        return False
    pl = path.lower()
    if pl.endswith(".py"):
        return has_prose_py(text)
    if pl.endswith((".rb", ".ex", ".exs", ".r", ".jl", ".pl", ".tf", ".yaml", ".yml")):
        if _line_family_prose(text, "#", HASH_DIR_OK):
            return True
        return has_prose_js(text)
    if pl.endswith((".lua", ".sql", ".hs")):
        if _line_family_prose(text, "--", DASH_DIR_OK):
            return True
        return has_prose_js(text)
    if pl.endswith((".clj", ".cljs", ".edn")):
        if _line_family_prose(text, ";", re.compile(r"^\s*$")):
            return True
        return has_prose_js(text)
    if pl.endswith((".erl", ".hrl")):
        if _line_family_prose(text, "%", re.compile(r"^\s*$")):
            return True
        return has_prose_js(text)
    if pl.endswith(".sql"):
        if re.search(r"(?m)^\s*--\s+\S", text):
            return True
        return has_prose_js(text)
    return has_prose_js(text)


def violates(path: str, text: str) -> bool:
    if not text:
        return False
    if not path:
        return text_has_prose(text, "")
    if not path_is_code(path):
        return False
    return text_has_prose(text, path)


def shell_targets_code(cmd: str) -> bool:
    if not cmd:
        return False
    return bool(
        REDIR_TO_CODE.search(cmd)
        or HEREDOC_TO_CODE.search(cmd)
        or EVAL_WRITE.search(cmd)
        or CODEISH.search(cmd)
    )


def shell_has_prose_payload(cmd: str) -> bool:
    if not cmd:
        return False
    if SHELL_BLOCK.search(cmd):
        return True
    if SHELL_JS_PROSE.search(cmd):
        return True
    if SHELL_PY_PROSE.search(cmd):
        return True
    return False


def _b64_decoded_prose(cmd: str) -> bool:
    m = BASE64_PIPE.search(cmd)
    if not m:
        return False
    target = m.group(2)
    if not CODEISH.search(target):
        return False
    tok = B64_TOKEN.search(cmd)
    if not tok:
        return False
    try:
        raw = base64.b64decode(tok.group(1) + "===")
        text = raw.decode("utf-8", "replace")
    except Exception:
        return False
    return text_has_prose(text, target)


def shell_write_class(cmd: str) -> str:
    if not cmd:
        return "allow"
    if _b64_decoded_prose(cmd):
        return "prose"
    if OPAQUE_WRITE.search(cmd):
        if re.search(r"\bsed\s+-n\b", cmd, re.I):
            return "allow"
        if re.search(r"prettier\s+[^\n]*--write", cmd, re.I):
            return "allow"
        if re.search(r"base64\s+[^\n]*-d[^\n]+\.(?:pem|crt|cer|der)\b", cmd, re.I):
            return "allow"
        if re.search(r"\bsed\s+[^\n]*\s-i\b[^\n]*\bs/", cmd, re.I):
            return "allow"
        if re.search(r"\b(?:git\s+apply|patch|applypatch)\b", cmd, re.I):
            return "opaque"
        if re.search(r"\bsed\s+[^\n]*\s-i\b", cmd, re.I):
            return "opaque"
        if CODEISH.search(cmd):
            return "opaque"
    if shell_targets_code(cmd) and shell_has_prose_payload(cmd):
        return "prose"
    return "allow"


def shell_prose_write(cmd: str) -> bool:
    return shell_write_class(cmd) == "prose"


def _py_comment_keep(body: str, line_no: int) -> bool:
    if body.startswith("#!") and line_no == 1:
        return True
    inner = body[1:].lstrip()
    if re.match(r"coding[:=]", inner, re.I):
        return True
    if PY_DIR_OK.match(inner):
        return True
    return not inner.strip()


def _js_comment_keep(node: str) -> bool:
    if node.startswith(_BL):
        inner = node[2:]
        if inner.endswith("*/"):
            inner = inner[:-2]
        return BLOCK_DIR_OK_MATCH(inner) or not inner.strip()
    if node.startswith(_SL):
        inner = node[2:]
        return JS_DIR_OK.match(inner) or not inner.strip()
    return True


def _strip_py(text: str) -> str | None:
    lines = text.splitlines(keepends=True)
    try:
        tokens = list(tokenize.generate_tokens(io.StringIO(text).readline))
    except tokenize.TokenError:
        return None
    for tok in tokens:
        if tok.type != tokenize.COMMENT:
            continue
        body = tok.string
        if _py_comment_keep(body, tok.start[0]):
            continue
        line_idx = tok.start[0] - 1
        if line_idx < 0 or line_idx >= len(lines):
            return None
        line = lines[line_idx]
        col = tok.start[1]
        prefix = line[:col]
        suffix = line[col + len(body):]
        if prefix.strip():
            return None
        lines[line_idx] = prefix + suffix
    out = "".join(lines)
    if text.endswith("\n"):
        return out if out.endswith("\n") else out + "\n"
    return out.rstrip("\n")


def _strip_js(text: str) -> str | None:
    spans = _js_comment_spans(text)
    for start, end, node in spans:
        if _js_comment_keep(node):
            continue
        if node.startswith(_BL):
            line_start = text.rfind("\n", 0, start) + 1
            prefix = text[line_start:start]
            if prefix.strip():
                return None
    remove: List[Tuple[int, int]] = []
    for start, end, node in spans:
        if not _js_comment_keep(node):
            remove.append((start, end))
    if not remove:
        return text
    chunks: List[str] = []
    pos = 0
    for start, end in sorted(remove):
        chunks.append(text[pos:start])
        pos = end
    chunks.append(text[pos:])
    return "".join(chunks)


def strip_prose(path: str, text: str) -> str | None:
    if not text:
        return text
    if not violates(path, text):
        return text
    stripped = _strip_js(text)
    if path.lower().endswith(".py"):
        stripped = _strip_py(text)
    if stripped is None:
        return None
    if violates(path, stripped):
        return None
    return stripped


def deny_payload() -> dict:
    return {
        "permission": "deny",
        "user_message": DENY_USER,
        "agent_message": DENY_AGENT,
    }


def ask_opaque_payload() -> dict:
    msg = (
        "Opaque shell write to code (patch/sed -i/interpreter redirect/base64). "
        "Confirm exact command; gate cannot inspect payload."
    )
    return {"permission": "ask", "user_message": msg, "agent_message": msg}
