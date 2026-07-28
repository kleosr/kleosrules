mod common;
use common::*;

#[test]
fn cli_gate_diff_passes() {
    let (code, stdout, stderr) = run_cli(&["gate-diff"]);
    assert_eq!(code, 0, "stdout={stdout} stderr={stderr}");
    assert!(stdout.contains("GATE_DIFF_PASS"), "{stdout}");
}

#[test]
fn cli_check_user_rules_runs() {
    let (code, _stdout, stderr) = run_cli(&["check-user-rules"]);
    assert!(
        code == 0 || code == 1,
        "unexpected code={code} stderr={stderr}"
    );
}

#[test]
fn cli_obedience_report_runs() {
    let (code, stdout, stderr) = run_cli(&["obedience-report"]);
    assert_eq!(code, 0, "stdout={stdout} stderr={stderr}");
    assert!(
        stdout.contains("no obedience log")
            || stdout.contains("decisions=")
            || stdout.contains("empty obedience"),
        "stdout={stdout}"
    );
}

#[test]
fn hooks_json_no_python3() {
    let hj = hooks_root().join("hooks.json");
    let text = std::fs::read_to_string(&hj).expect("hooks.json");
    assert!(!text.contains("python3"), "hooks.json still references python3");
    assert!(text.contains("kleos-gate"), "hooks.json missing kleos-gate");
    let cfg: serde_json::Value = serde_json::from_str(&text).expect("json");
    let hooks = cfg.get("hooks").expect("hooks");
    for (event, entries) in hooks.as_object().expect("obj") {
        for entry in entries.as_array().expect("arr") {
            let cmd = entry.get("command").and_then(|v| v.as_str()).unwrap_or("");
            assert!(
                cmd.contains("kleos-gate"),
                "{event} not on kleos-gate: {cmd}"
            );
        }
    }
}

#[test]
fn project_hooks_json_no_python3() {
    let hj = hooks_root().join("hooks.project.json");
    let text = std::fs::read_to_string(&hj).expect("hooks.project.json");
    assert!(!text.contains("python3"));
    assert!(text.contains("kleos-gate"));
}
