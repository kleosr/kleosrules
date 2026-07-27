use std::fs;
use std::path::Path;
use std::process;

use super::sync::sync_hooks;
use super::util::{
    chmod_x, ensure_gate_bin, force_skills, home_cursor, is_symlink, load_lines, pack_root, say,
    symlink_force,
};

pub fn install_user_hooks(pack: &Path) -> Result<(), String> {
    ensure_gate_bin(pack)?;
    let dest = home_cursor();
    fs::create_dir_all(dest.join("hooks/bin")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dest.join("hooks/policy")).map_err(|e| e.to_string())?;
    let bin_dst = dest.join("hooks/bin/kleos-gate");
    fs::copy(pack.join("hooks/bin/kleos-gate"), &bin_dst).map_err(|e| e.to_string())?;
    chmod_x(&bin_dst);
    for name in ["shell.json", "lean.json", "ask-scope.json", "secrets.json"] {
        fs::copy(
            pack.join("hooks/policy").join(name),
            dest.join("hooks/policy").join(name),
        )
        .map_err(|e| e.to_string())?;
    }
    fs::copy(pack.join("hooks/hooks.json"), dest.join("hooks.json")).map_err(|e| e.to_string())?;
    say(&format!("[ok] hooks → {}/hooks.json (kleos-gate)", dest.display()));
    Ok(())
}

fn install_global_rules(pack: &Path) -> Result<(), String> {
    let dest = home_cursor().join("rules");
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    fs::copy(
        pack.join("user-rules/option-c-core.mdc"),
        dest.join("option-c-core.mdc"),
    )
    .map_err(|e| e.to_string())?;
    for f in ["native-lean-autoload", "ponytail", "lean-code", "agent"] {
        fs::copy(
            pack.join("project-rules").join(format!("{f}.mdc")),
            dest.join(format!("{f}.mdc")),
        )
        .map_err(|e| e.to_string())?;
        say(&format!("[ok]  ~/.cursor/rules/{f}.mdc (alwaysApply)"));
    }
    Ok(())
}

fn install_skills(pack: &Path) -> Result<(), String> {
    let personal = home_cursor().join("skills");
    fs::create_dir_all(&personal).map_err(|e| e.to_string())?;
    for skill in load_lines(&pack.join("config/skills.txt")) {
        let src = pack.join("skills").join(&skill);
        if !src.join("SKILL.md").is_file() {
            return Err(format!("[fail] missing {}/SKILL.md", src.display()));
        }
        let dst = personal.join(&skill);
        if dst.exists() && !is_symlink(&dst) {
            if force_skills() {
                let _ = fs::remove_dir_all(&dst);
                say(&format!("[force] replaced: {skill}"));
            } else {
                say(&format!(
                    "[warn] skip non-symlink: {}  (FORCE_SKILLS=1 to replace)",
                    dst.display()
                ));
                continue;
            }
        }
        symlink_force(&src, &dst)?;
        say(&format!("[ok]  {skill}"));
    }
    Ok(())
}

pub fn run_hooks(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    if let Err(e) = install_user_hooks(&pack) {
        super::util::die(&e);
    }
    process::exit(0);
}

pub fn run(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    let dest = home_cursor();
    fs::create_dir_all(dest.join("rules")).ok();
    fs::create_dir_all(dest.join("skills")).ok();
    fs::create_dir_all(dest.join("hooks")).ok();
    say("==> Hooks");
    if let Err(e) = install_user_hooks(&pack) {
        super::util::die(&e);
    }
    if let Err(e) = sync_hooks(&pack) {
        super::util::die(&e);
    }
    if let Err(e) = super::sync::link_pack_rules(&pack) {
        super::util::die(&e);
    }
    say("==> Rules mirror (always-on global companions)");
    if let Err(e) = install_global_rules(&pack) {
        super::util::die(&e);
    }
    say("==> Skills (symlinks → pack/skills)");
    if let Err(e) = install_skills(&pack) {
        super::util::die(&e);
    }
    say("");
    say("==> USER RULES (manual paste required)");
    say("    Cursor → Settings → Rules → User Rules");
    say(&format!(
        "    Paste: {}/user-rules/USER-RULES.paste.txt",
        pack.display()
    ));
    say("");
    say("==> Vernacular (per app repo — soft until alwaysApply contract exists)");
    say("    mkdir -p .cursor/rules");
    say(&format!(
        "    cp \"{}/skills/vernacular/TEMPLATE.md\" .cursor/rules/vernacular.mdc",
        pack.display()
    ));
    say("");
    say(&format!("[done] {}", pack.display()));
    say("       New chat after User Rules paste.");
    process::exit(0);
}
