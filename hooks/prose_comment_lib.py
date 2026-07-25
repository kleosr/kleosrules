#!/usr/bin/env python3
from __future__ import annotations

import io
import re
import tokenize
from typing import List

CODE_EXT = (
    ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs",
    ".vue", ".svelte",
    ".py", ".go", ".rs", ".java", ".kt", ".swift",
    ".rb", ".php", ".cs", ".cpp", ".cc", ".c", ".h", ".hpp",
    ".sql",
)
SKIP_EXT = (
    ".sh", ".bash", ".zsh", ".fish", ".ps1",
    ".md", ".mdc", ".txt", ".json", ".yml", ".yaml", ".toml",
    ".lock", ".svg", ".html", ".css", ".scss",
)

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
    r"^\s*(?:type:\s*ignore|noqa|pragma:|pylint:|mypy:|fmt:|ruff:)",
    re.I,
)
BLOCK_DIR_OK = re.compile(
    r"^\s*(?:eslint|prettier|istanbul|biome|ts-|@ts-)",
    re.I,
)

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
    r"#" + r"(?!\s*(?:type:\s*ignore|noqa|pragma:|pylint:|mypy:|fmt:|ruff:|!|/))"
)
SHELL_BLOCK = re.compile(re.escape(_BL))

_EXT_GROUP = "|".join(re.escape(e) for e in CODE_EXT)
REDIR_TO_CODE = re.compile(
    rf"(?:>>?|tee(?:\s+-a)?)\s*[\"']?[^\s\"';]+(?:{_EXT_GROUP})\b",
    re.I,
)
HEREDOC_TO_CODE = re.compile(
    rf"cat\s*>+\s*[\"']?[^\s\"';]+(?:{_EXT_GROUP})",
    re.I,
)
EVAL_WRITE = re.compile(
    rf"(?:python3?|node|deno)\s+-c\b.*(?:"
    rf"open\s*\(|write_text\s*\(|write_bytes\s*\(|Path\s*\([^)]*\)\s*\.\s*write_text|\.write\s*\()"
    rf".*(?:{_EXT_GROUP})",
    re.I | re.S,
)


def path_is_code(path: str) -> bool:
    p = (path or "").split("?")[0].lower()
    if not p:
        return False
    if any(p.endswith(ext) for ext in SKIP_EXT):
        return False
    return any(p.endswith(ext) for ext in CODE_EXT)


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


def _js_comment_nodes(text: str) -> List[str]:
    nodes: List[str] = []
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
                        if text[i] in "\"'":
                            q = text[i]
                            i += 1
                            while i < n:
                                if text[i] == "\\":
                                    i += 2
                                    continue
                                if text[i] == q:
                                    i += 1
                                    break
                                i += 1
                            continue
                        if text[i] == "`":
                            i += 1
                            while i < n:
                                if text[i] == "\\":
                                    i += 2
                                    continue
                                if text[i] == "`":
                                    i += 1
                                    break
                                i += 1
                            continue
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
            nodes.append(text[start:i])
            continue

        if c == "/" and nxt == "*":
            start = i
            i += 2
            while i + 1 < n and not (text[i] == "*" and text[i + 1] == "/"):
                i += 1
            i = min(i + 2, n)
            nodes.append(text[start:i])
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
    return nodes


def _looks_like_regex(text: str, i: int) -> bool:
    j = i - 1
    while j >= 0 and text[j] in " \t":
        j -= 1
    if j < 0:
        return True
    prev = text[j]
    if prev in "=(!&|:?,;{[>~+-*%^":
        return True
    if text[max(0, j - 5) : j + 1].endswith("return"):
        return True
    return False


def _js_prose_from_nodes(nodes: List[str]) -> bool:
    for node in nodes:
        if node.startswith(_SL):
            inner = node[2:]
            if not inner.strip():
                continue
            if JS_DIR_OK.match(inner):
                continue
            return True
        if node.startswith(_BL):
            body = node[2:]
            if body.endswith("*" + "/"):
                body = body[:-2]
            if not body.strip():
                continue
            if BLOCK_DIR_OK.match(body):
                continue
            return True
    return False


def has_prose_py(text: str) -> bool:
    return bool(_py_comment_nodes(text))


def has_prose_js(text: str) -> bool:
    return _js_prose_from_nodes(_js_comment_nodes(text))


def text_has_prose(text: str, path: str = "") -> bool:
    if not text:
        return False
    if path.lower().endswith(".py"):
        return has_prose_py(text)
    if path.lower().endswith(".sql"):
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


def shell_prose_write(cmd: str) -> bool:
    return shell_targets_code(cmd) and shell_has_prose_payload(cmd)


def deny_payload() -> dict:
    return {
        "permission": "deny",
        "user_message": DENY_USER,
        "agent_message": DENY_AGENT,
    }
