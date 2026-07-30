TOOLCHAIN — kleosr V2

Requires: bash, jq

Smoke
chmod +x hooks/session_start.sh hooks/before_submit_prompt.sh hooks/stop_gate.sh
bash -n hooks/session_start.sh hooks/before_submit_prompt.sh hooks/stop_gate.sh
echo '{}' | hooks/session_start.sh | jq -e .additional_context
echo '{"prompt":"fix the gate"}' | hooks/before_submit_prompt.sh | jq -e .additional_context
echo '{"status":"completed"}' | hooks/stop_gate.sh | jq -e .followup_message
echo '{"status":"completed","text":"INTENT: x\nDone-when: y"}' | hooks/stop_gate.sh | jq -e .followup_message

Green means jq keys present. Residual: no former Rust cargo suites.
