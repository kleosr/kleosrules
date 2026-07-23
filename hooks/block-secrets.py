#!/usr/bin/env python3
import json, re, sys
SECRET_RE = re.compile(
    r"(?i)(sk_live_[A-Za-z0-9]{16,}|sk_test_[A-Za-z0-9]{16,}|"
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|"
    r"xox[baprs]-[A-Za-z0-9-]{20,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----)"
)
def out(d):
    print(json.dumps(d)); raise SystemExit(0)
try:
    data = json.load(sys.stdin)
except Exception:
    out({"permission": "allow"})
blobs = []
for k in ("prompt", "command", "content", "new_string", "old_string"):
    v = data.get(k)
    if isinstance(v, str):
        blobs.append(v)
inp = data.get("input") or data.get("tool_input") or {}
if isinstance(inp, dict):
    for k, v in inp.items():
        if k != "path" and isinstance(v, str):
            blobs.append(v)
text = "\n".join(blobs)
if text and SECRET_RE.search(text) and "EXAMPLE_SECRET" not in text:
    out({"permission": "deny",
         "user_message": "Blocked secret-like material (lab gate).",
         "agent_message": "Blocked secret-like material (lab gate)."})
out({"permission": "allow"})
