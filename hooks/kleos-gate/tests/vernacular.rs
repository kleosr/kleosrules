mod common;
use common::*;
use serde_json::json;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn check_content_path_vernacular_denies() {
    let root = tempfile_dir();
    std::fs::create_dir_all(root.join("project-rules")).unwrap();
    std::fs::create_dir_all(root.join("hooks/kleos-gate")).unwrap();
    std::fs::write(
        root.join("project-rules/vernacular.mdc"),
        "file_name_pattern: pack_native\nallowed_path_prefixes: hooks/\n",
    )
    .unwrap();
    let mut cmd = Command::new(bin_path());
    cmd.arg("--check-content")
        .arg("--path")
        .arg("src/FooUseCase.rs")
        .current_dir(root.join("hooks/kleos-gate"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", tempfile_dir());
    let mut child = cmd.spawn().expect("spawn check-content");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin.write_all(b"pub struct X {}\n").expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(code, 2, "{err}");
    assert!(
        err.contains("vernacular") || err.contains("file-name") || err.contains("path"),
        "{err}"
    );
}

#[test]
fn write_forbidden_class_suffix_denies() {
    let root = tempfile_dir();
    let state = tempfile_dir();
    seed_recall(&policy_dir(), &state, "vn1");
    std::fs::create_dir_all(root.join("project-rules")).unwrap();
    std::fs::create_dir_all(root.join("hooks")).unwrap();
    std::fs::write(
        root.join("project-rules/vernacular.mdc"),
        "file_name_pattern: pack_native\nallowed_path_prefixes: hooks/\nforbidden_class_suffixes: UseCase, Repository\nclass_pattern: PascalCase\n",
    )
    .unwrap();
    let path = root.join("hooks/pay.rs");
    let body = "export class InvoiceUseCase {}\n";
    let mut cmd = Command::new(bin_path());
    cmd.arg("write")
        .current_dir(&root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("KLEOS_HOOKS_DIR", hooks_root())
        .env("KLEOS_POLICY_DIR", policy_dir())
        .env("KLEOS_STATE_DIR", &state);
    let mut child = cmd.spawn().expect("spawn");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin
            .write_all(
                json!({
                    "conversation_id": "vn1",
                    "tool_name": "Write",
                    "tool_input": {
                        "path": path.to_string_lossy(),
                        "contents": body
                    }
                })
                .to_string()
                .as_bytes(),
            )
            .expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    let code = out.status.code().unwrap_or(1);
    let obj: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim()).unwrap_or(json!({}));
    assert_eq!(code, 2, "{obj}");
    assert_eq!(perm(&obj), "deny");
}
