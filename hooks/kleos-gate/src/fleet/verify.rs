use std::fs;
use std::path::Path;
use std::process;

use super::discover::discover;
use super::util::{
    files_equal, home_cursor, is_executable, is_symlink, load_lines, pack_root, say, HOOK_NEED,
    SHARED,
};

fn check_shared_dest(
    pack: &Path,
    dest: &Path,
    label: &str,
    mode: &str,
    fail: &mut i32,
) {
    if !dest.is_dir() {
        say(&format!("[MISSING] {label}/.cursor/rules"));
        *fail = 1;
        return;
    }
    let rules = pack.join("project-rules");
    for name in SHARED {
        let src = rules.join(format!("{name}.mdc"));
        let dst = dest.join(format!("{name}.mdc"));
        if !dst.exists() && !is_symlink(&dst) {
            say(&format!("[MISSING] {label}/{name}.mdc"));
            *fail = 1;
            continue;
        }
        if mode == "symlink" {
            if !is_symlink(&dst) || !files_equal(&src, &dst) {
                say(&format!("[BAD] {label}/{name}.mdc symlink"));
                *fail = 1;
            }
        } else if is_symlink(&dst) || !files_equal(&src, &dst) {
            say(&format!("[DRIFT] {label}/{name}.mdc"));
            *fail = 1;
        }
    }
    for orphan in load_lines(&pack.join("config/retired.txt")) {
        let p = dest.join(&orphan);
        if p.exists() || is_symlink(&p) {
            say(&format!("[ORPHAN] {label}/{orphan}"));
            *fail = 1;
        }
    }
}

fn check_project_hooks(pack: &Path, root: &Path, label: &str, fail: &mut i32) {
    let hj = root.join(".cursor/hooks.json");
    let hd = root.join(".cursor/hooks");
    if !hj.is_file() {
        say(&format!("[MISSING] {label}/.cursor/hooks.json"));
        *fail = 1;
        return;
    }
    let Ok(text) = fs::read_to_string(&hj) else {
        say(&format!("[DRIFT] {label}/.cursor/hooks.json unreadable"));
        *fail = 1;
        return;
    };
    if !text.contains("kleos-gate") {
        say(&format!("[DRIFT] {label} hooks.json missing kleos-gate"));
        *fail = 1;
    }
    if !text.contains("Write|StrReplace|EditNotebook") {
        say(&format!("[DRIFT] {label} hooks.json write matcher"));
        *fail = 1;
    }
    if text.contains("python3") {
        say(&format!("[DRIFT] {label} hooks.json still references python3"));
        *fail = 1;
    }
    if !hd.is_dir() {
        say(&format!("[MISSING] {label}/.cursor/hooks"));
        *fail = 1;
        return;
    }
    for f in HOOK_NEED {
        if !hd.join(f).exists() {
            say(&format!("[MISSING] {label}/.cursor/hooks/{f}"));
            *fail = 1;
        }
    }
    let bin = hd.join("bin/kleos-gate");
    if bin.is_file() && !is_executable(&bin) {
        say(&format!("[DRIFT] {label} kleos-gate not executable"));
        *fail = 1;
    }
    if !files_equal(&pack.join("hooks/hooks.project.json"), &hj) {
        say(&format!("[DRIFT] {label}/.cursor/hooks.json"));
        *fail = 1;
    }
}

pub fn verify(pack: &Path) -> Result<(), String> {
    let mut fail = 0i32;
    check_shared_dest(pack, &pack.join(".cursor/rules"), "pack", "symlink", &mut fail);
    let repos = discover(pack);
    say(&format!("[scan] verifying {} project(s)", repos.len()));
    let pack_c = pack.canonicalize().unwrap_or_else(|_| pack.to_path_buf());
    for repo in &repos {
        if *repo == pack_c {
            continue;
        }
        let label = repo
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("repo");
        check_shared_dest(pack, &repo.join(".cursor/rules"), label, "copy", &mut fail);
        check_project_hooks(pack, repo, label, &mut fail);
    }
    check_project_hooks(pack, pack, "pack", &mut fail);
    let personal = home_cursor().join("skills");
    for skill in load_lines(&pack.join("config/skills.txt")) {
        let src = pack.join("skills").join(&skill);
        let dst = personal.join(&skill);
        if !src.join("SKILL.md").is_file() {
            say(&format!("[MISSING] skills/{skill}/SKILL.md"));
            fail = 1;
        } else if !is_symlink(&dst) {
            say(&format!("[BAD-LINK] ~/.cursor/skills/{skill}"));
            fail = 1;
        } else {
            let src_c = src.canonicalize().unwrap_or(src);
            let dst_c = dst.canonicalize().unwrap_or(dst);
            if src_c != dst_c {
                say(&format!("[BAD-LINK] ~/.cursor/skills/{skill}"));
                fail = 1;
            }
        }
    }
    for req in [
        "user-rules/USER-RULES.paste.txt",
        "user-rules/option-c-core.mdc",
        "project-rules/agent.mdc",
        "project-rules/native-lean-autoload.mdc",
        "project-rules/ponytail.mdc",
        "project-rules/lean-code.mdc",
        "project-rules/vernacular.mdc",
        "hooks/hooks.json",
        "hooks/hooks.project.json",
        "hooks/bin/kleos-gate",
        "hooks/policy/shell.json",
        "hooks/policy/lean.json",
        "hooks/policy/secrets.json",
        "hooks/policy/ask-scope.json",
        "hooks/kleos-gate/Cargo.toml",
        "hooks/kleos-gate/tests/integration.rs",
        "config/skills.txt",
    ] {
        if !pack.join(req).exists() {
            say(&format!("[MISSING] {req}"));
            fail = 1;
        }
    }
    for junk in [
        "agent.mdc",
        "types.mdc",
        "USER-RULES.paste.txt",
        "option-c-core.mdc",
        "skills.txt",
        "scan.roots",
        "glob",
    ] {
        if pack.join(junk).exists() {
            say(&format!("[ROOT-MESS] {junk} belongs in a subfolder"));
            fail = 1;
        }
    }
    if fail == 0 {
        say("[PASS] organized pack OK");
        Ok(())
    } else {
        say("[FAIL] organization or sync broken");
        Err("verify failed".into())
    }
}

pub fn run(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    match verify(&pack) {
        Ok(()) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
