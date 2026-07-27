use std::fs;
use std::path::Path;
use std::process;

use super::util::{is_executable, pack_root, say};

pub fn run(hooks: &Path) -> ! {
    let pack = pack_root(hooks);
    let bin = pack.join("hooks/bin/kleos-gate");
    if !is_executable(&bin) {
        super::util::die(&format!(
            "missing {} — run cargo build --release first",
            bin.display()
        ));
    }
    let hook = pack.join(".git/hooks/pre-commit");
    if let Some(parent) = hook.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let body = "#!/usr/bin/env bash\nset -euo pipefail\nROOT=\"$(git rev-parse --show-toplevel)\"\nexport KLEOS_HOOKS_DIR=\"$ROOT/hooks\"\nexport KLEOS_POLICY_DIR=\"$ROOT/hooks/policy\"\nexec \"$ROOT/hooks/bin/kleos-gate\" gate-diff\n";
    if let Err(e) = fs::write(&hook, body) {
        super::util::die(&format!("write pre-commit: {e}"));
    }
    super::util::chmod_x(&hook);
    say("[ok] pre-commit → hooks/bin/kleos-gate gate-diff");
    process::exit(0);
}
