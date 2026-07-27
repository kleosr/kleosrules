use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SHARED: &[&str] = &[
    "agent",
    "types",
    "testing",
    "debugging",
    "native-lean-autoload",
    "ponytail",
    "lean-code",
    "obsidian-memory",
    "context-curator",
];

pub const HOOK_NEED: &[&str] = &[
    "bin/kleos-gate",
    "policy/shell.json",
    "policy/lean.json",
    "policy/secrets.json",
    "policy/ask-scope.json",
    "policy/context.json",
];

pub fn pack_root(hooks: &Path) -> PathBuf {
    if hooks.file_name().and_then(|s| s.to_str()) == Some("hooks") {
        hooks
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| hooks.to_path_buf())
    } else {
        hooks.to_path_buf()
    }
}

pub fn home_cursor() -> PathBuf {
    env::var_os("HOME")
        .map(|h| PathBuf::from(h).join(".cursor"))
        .unwrap_or_else(|| PathBuf::from("/tmp/.cursor"))
}

pub fn force_skills() -> bool {
    matches!(env::var("FORCE_SKILLS").as_deref(), Ok("1"))
}

pub fn load_lines(path: &Path) -> Vec<String> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|s| s.to_string())
        .collect()
}

pub fn files_equal(a: &Path, b: &Path) -> bool {
    match (fs::read(a), fs::read(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

pub fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn chmod_x(path: &Path) {
    if let Ok(meta) = fs::metadata(path) {
        let mut p = meta.permissions();
        p.set_mode(p.mode() | 0o755);
        let _ = fs::set_permissions(path, p);
    }
}

pub fn ensure_gate_bin(pack: &Path) -> Result<(), String> {
    let bin = pack.join("hooks/bin/kleos-gate");
    if is_executable(&bin) {
        return Ok(());
    }
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(pack.join("hooks/kleos-gate"))
        .status()
        .map_err(|e| format!("cargo: {e}"))?;
    if !status.success() {
        return Err("cargo build --release failed".into());
    }
    fs::create_dir_all(pack.join("hooks/bin")).map_err(|e| e.to_string())?;
    let built = pack.join("hooks/kleos-gate/target/release/kleos-gate");
    fs::copy(&built, &bin).map_err(|e| e.to_string())?;
    chmod_x(&bin);
    Ok(())
}

pub fn symlink_force(src: &Path, dst: &Path) -> Result<(), String> {
    if dst.exists() || is_symlink(dst) {
        fs::remove_file(dst).map_err(|e| e.to_string())?;
    }
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    symlink(src, dst).map_err(|e| e.to_string())
}

pub fn say(msg: &str) {
    let _ = writeln!(io::stdout(), "{msg}");
}

pub fn die(msg: &str) -> ! {
    let _ = writeln!(io::stderr(), "{msg}");
    std::process::exit(1);
}
