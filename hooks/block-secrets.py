#!/usr/bin/env python3
import re, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import emit, load_stdin, walk_strings
SECRET_RE = re.compile(
    r"(?i)(sk_live_[A-Za-z0-9]{16,}|sk_test_[A-Za-z0-9]{16,}|"
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|"
    r"xox[baprs]-[A-Za-z0-9-]{20,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----)"
)
MSG = "Blocked secret-like material (lab gate)."
def allow(ev):
    emit({} if ev == "beforeSubmitPrompt" else {"permission": "allow"})
def block(ev):
    if ev == "beforeSubmitPrompt":
        emit({"continue": False, "user_message": MSG}, 2)
    emit({"permission": "deny", "user_message": MSG, "agent_message": MSG}, 2)
data = load_stdin()
event = str(data.get("hook_event_name") or "")
text = "\n".join(walk_strings(data))
if text and SECRET_RE.search(text) and "EXAMPLE_SECRET" not in text:
    block(event)
allow(event)
