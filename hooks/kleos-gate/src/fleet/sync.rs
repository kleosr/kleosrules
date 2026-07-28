use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use super::discover::discover;
use super::util::{
    chmod_x, ensure_gate_bin, force_skills, home_cursor, is_symlink, load_lines, pack_root, say,
    symlink_force, SHARED,
};

fn sync_hooks_into(pack: &Path, root: &Path, label: &str) -> Result<(), String> {
    let dest = root.join(".cursor/hooks");
    fs::create_dir_all(dest.join("bin")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dest.join("policy")).map_err(|e| e.to_string())?;
    let bin_src = pack.join("hooks/bin/kleos-gate");
    let bin_dst = dest.join("bin/kleos-gate");
    fs::copy(&bin_src, &bin_dst).map_err(|e| e.to_string())?;
    chmod_x(&bin_dst);
    for name in [
        "shell.json",
        "lean.json",
        "ask-scope.json",
        "secrets.json",
        "context.json",
        "delete.json",
    ] {
        fs::copy(
            pack.join("hooks/policy").join(name),
            dest.join("policy").join(name),
        )
        .map_err(|e| e.to_string())?;
    }
    fs::copy(
        pack.join("hooks/hooks.project.json"),
        root.join(".cursor/hooks.json"),
    )
    .map_err(|e| e.to_string())?;
    say(&format!("[ok]  hooks → {label} (kleos-gate)"));
    Ok(())
}

pub fn sync_hooks(pack: &Path) -> Result<(), String> {
    ensure_gate_bin(pack)?;
    sync_hooks_into(pack, pack, "pack")?;
    let pack_c = pack.canonicalize().unwrap_or_else(|_| pack.to_path_buf());
    for repo in discover(pack) {
        if repo == pack_c {
            continue;
        }
        let label = repo
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("repo");
        sync_hooks_into(pack, &repo, label)?;
    }
    Ok(())
}

fn link_ssot_cursor(pack: &Path) -> Result<(), String> {
    let dest = pack.join(".cursor/rules");
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    let rules = pack.join("project-rules");
    for name in SHARED {
        let src = rules.join(format!("{name}.mdc"));
        if !src.is_file() {
            return Err(format!("missing {}", src.display()));
        }
        let dst = dest.join(format!("{name}.mdc"));
        let target = PathBuf::from(format!("../../project-rules/{name}.mdc"));
        symlink_force(&target, &dst)?;
    }
    let vern_src = rules.join("vernacular.mdc");
    if vern_src.is_file() {
        let dst = dest.join("vernacular.mdc");
        let target = PathBuf::from("../../project-rules/vernacular.mdc");
        symlink_force(&target, &dst)?;
    }
    for orphan in load_lines(&pack.join("config/retired.txt")) {
        let p = dest.join(&orphan);
        if p.exists() || is_symlink(&p) {
            let _ = fs::remove_file(&p);
            say(&format!("[rm]  {orphan}"));
        }
    }
    say("[ok]  pack .cursor/rules → project-rules/");
    Ok(())
}

pub fn link_pack_rules(pack: &Path) -> Result<(), String> {
    link_ssot_cursor(pack)
}

fn sync_into(pack: &Path, dest: &Path, label: &str) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let rules = pack.join("project-rules");
    for name in SHARED {
        let src = rules.join(format!("{name}.mdc"));
        if !src.is_file() {
            return Err(format!("missing {}", src.display()));
        }
        let dst = dest.join(format!("{name}.mdc"));
        if is_symlink(&dst) {
            let _ = fs::remove_file(&dst);
        }
        fs::copy(&src, &dst).map_err(|e| e.to_string())?;
    }
    for orphan in load_lines(&pack.join("config/retired.txt")) {
        let p = dest.join(&orphan);
        if p.exists() || is_symlink(&p) {
            let _ = fs::remove_file(&p);
            say(&format!("[rm]  {label}/{orphan}"));
        }
    }
    say(&format!("[ok]  {label}"));
    Ok(())
}

fn link_personal_skills(pack: &Path) -> Result<(), String> {
    let personal = home_cursor().join("skills");
    fs::create_dir_all(&personal).map_err(|e| e.to_string())?;
    for skill in load_lines(&pack.join("config/skills.txt")) {
        let src = pack.join("skills").join(&skill);
        if !src.join("SKILL.md").is_file() {
            return Err(format!("missing {}/SKILL.md", src.display()));
        }
        let dst = personal.join(&skill);
        if dst.exists() && !is_symlink(&dst) {
            if force_skills() {
                let _ = fs::remove_dir_all(&dst);
            } else {
                say(&format!("[warn] skip non-symlink: {}", dst.display()));
                continue;
            }
        }
        symlink_force(&src, &dst)?;
    }
    for skill in load_lines(&pack.join("config/retired-skills.txt")) {
        let dst = personal.join(skill);
        if is_symlink(&dst) {
            let _ = fs::remove_file(&dst);
        }
    }
    say("[ok]  skills → pack/skills");
    Ok(())
}

pub fn sync_rules(pack: &Path) -> Result<(), String> {
    let repos = discover(pack);
    say(&format!("[scan] discovered {} project(s)", repos.len()));
    for r in &repos {
        say(&format!("[scan]  - {}", r.display()));
    }
    link_personal_skills(pack)?;
    link_ssot_cursor(pack)?;
    let pack_c = pack.canonicalize().unwrap_or_else(|_| pack.to_path_buf());
    for repo in repos {
        if repo == pack_c {
            continue;
        }
        let label = repo
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("repo")
            .to_string();
        sync_into(pack, &repo.join(".cursor/rules"), &label)?;
    }
    Ok(())
}

pub fn run_hooks(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    if let Err(e) = sync_hooks(&pack) {
        super::util::die(&e);
    }
    process::exit(0);
}

pub fn run(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    if let Err(e) = sync_rules(&pack) {
        super::util::die(&e);
    }
    if let Err(e) = sync_hooks(&pack) {
        super::util::die(&e);
    }
    if let Err(e) = super::verify::verify(&pack) {
        super::util::die(&e);
    }
    say("=== done ===");
    process::exit(0);
}
