#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hookio import emit, load_stdin, log_decision, path_from  # noqa: E402

SECRET_NAME = re.compile(
    r"(?i)(^|/|\\)"
    r"("
    r"\.env($|\.)|"
    r"\.env\.[^/]+$|"
    r"id_rsa$|id_dsa$|id_ecdsa$|id_ed25519$|"
    r"[^/]+\.pem$|[^/]+\.key$|"
    r"secrets?\.(ya?ml|json|toml)$|"
    r"credentials\.(ya?ml|json)$|"
    r"\.aws/credentials$|"
    r"terraform\.tfstate(\.backup)?$"
    r")"
)
ALLOW_NAME = re.compile(
    r"(?i)(\.env\.example|\.env\.sample|\.sample\.|\.template\.|"
    r"example\.(ya?ml|json)|sample\.(ya?ml|json))"
)
MSG = "Blocked read of secret-like path into context."


def main() -> None:
    data = load_stdin()
    if data.get("_parse_error"):
        emit({"permission": "deny", "user_message": "gate-read parse error"}, 2)
    path = path_from(data) or str(data.get("file_path") or data.get("path") or "")
    event = str(data.get("hook_event_name") or "beforeReadFile")
    if not path:
        emit({"permission": "allow"})
    if ALLOW_NAME.search(path):
        emit({"permission": "allow"})
    if SECRET_NAME.search(path.replace("\\", "/")):
        log_decision("gate-read", event, "deny", path)
        if event == "beforeTabFileRead":
            emit({"permission": "deny"}, 2)
        emit({"permission": "deny", "user_message": MSG}, 2)
    emit({"permission": "allow"})


if __name__ == "__main__":
    try:
        main()
    except SystemExit:
        raise
    except Exception:
        emit({"permission": "deny", "user_message": "gate-read failed closed"}, 2)
