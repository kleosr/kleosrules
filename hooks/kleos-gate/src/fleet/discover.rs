use std::path::{Path, PathBuf};
use std::process;

use super::util::{load_lines, pack_root, say};

fn is_ignored(pack: &Path, path: &Path) -> bool {
    let ignore = load_lines(&pack.join("config/scan.ignore"));
    let pack_canon = path_canon(pack);
    let path_canon = path_canon(path);
    if pack_canon == path_canon {
        return true;
    }
    let base = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let path_s = path.to_string_lossy();
    ignore.iter().any(|pat| base == pat || path_s.contains(pat))
}

fn path_canon(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}

fn is_project(d: &Path) -> bool {
    if !d.is_dir() {
        return false;
    }
    d.join(".git").is_dir()
        || d.join("package.json").is_file()
        || d.join("pnpm-workspace.yaml").is_file()
        || d.join("Cargo.toml").is_file()
        || d.join("go.mod").is_file()
        || d.join("pyproject.toml").is_file()
        || d.join("AGENTS.md").is_file()
        || d.join(".cursor/rules").is_dir()
}

pub fn discover(pack: &Path) -> Vec<PathBuf> {
    let roots_file = pack.join("config/scan.roots");
    let lines = load_lines(&roots_file);
    let roots: Vec<PathBuf> = if lines.is_empty() {
        pack.parent()
            .map(|p| vec![p.to_path_buf()])
            .unwrap_or_default()
    } else {
        lines.into_iter().map(PathBuf::from).collect()
    };
    let mut out = Vec::new();
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        if is_project(&root) && !is_ignored(pack, &root) {
            out.push(path_canon(&root));
        }
        let Ok(rd) = std::fs::read_dir(&root) else {
            continue;
        };
        for ent in rd.flatten() {
            let child = ent.path();
            if !child.is_dir() {
                continue;
            }
            if is_ignored(pack, &child) {
                continue;
            }
            if is_project(&child) {
                out.push(path_canon(&child));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn run(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    for p in discover(&pack) {
        say(&p.display().to_string());
    }
    process::exit(0);
}
