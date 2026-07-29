use crate::policy::LeanPolicy;
use std::env;
use std::fs;
use std::path::Path;

pub fn enabled(pol: &LeanPolicy) -> bool {
    if pol.enforce_always {
        return true;
    }
    match env::var(&pol.enabled_env) {
        Ok(v) => !(v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("no")),
        Err(_) => pol.enabled_default,
    }
}

pub fn is_code_path(path: &str, pol: &LeanPolicy) -> bool {
    let lower = path.to_lowercase();
    pol.code_extensions.iter().any(|ext| lower.ends_with(ext))
}

fn line_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let n = text.matches('\n').count();
    if text.ends_with('\n') {
        n
    } else {
        n + 1
    }
}

fn lim_new(pol: &LeanPolicy) -> usize {
    env::var(&pol.new_file_loc_env)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(pol.new_file_loc)
}

fn lim_delta(pol: &LeanPolicy) -> usize {
    env::var(&pol.net_delta_env)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(pol.net_delta)
}

fn lim_abs(pol: &LeanPolicy) -> usize {
    env::var(&pol.file_loc_max_env)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(pol.file_loc_max)
}

pub fn check(path: &str, contents: Option<&str>, old: Option<&str>, new: Option<&str>, pol: &LeanPolicy) -> Option<String> {
    if !enabled(pol) || !is_code_path(path, pol) {
        return None;
    }
    let abs = lim_abs(pol);
    if let Some(body) = contents {
        let n = line_count(body);
        if n > abs {
            return Some(format!(
                "Lean meter: post-write file {n} LOC > absolute {abs} (split by responsibility or KLEOS_LEAN_FILE_LOC_MAX)"
            ));
        }
        let p = Path::new(path);
        if !p.is_file() {
            let lim = lim_new(pol);
            if n > lim {
                return Some(format!(
                    "Lean meter: new file {n} LOC > {lim} (reuse/split or KLEOS_LEAN=0 / KLEOS_LEAN_NEW_FILE_LOC)"
                ));
            }
        } else {
            let old_n = fs::read_to_string(p)
                .map(|t| line_count(&t))
                .unwrap_or(0);
            let delta = n.saturating_sub(old_n);
            let lim = lim_delta(pol);
            if delta > lim {
                return Some(format!(
                    "Lean meter: Write net +{delta} LOC > {lim} (reuse/split or KLEOS_LEAN=0 / KLEOS_LEAN_NET_DELTA)"
                ));
            }
        }
    }
    if let (Some(o), Some(n)) = (old, new) {
        let cur = fs::read_to_string(Path::new(path))
            .map(|t| line_count(&t))
            .unwrap_or(0) as isize;
        let projected = (cur + line_count(n) as isize - line_count(o) as isize).max(0);
        if projected > abs as isize {
            return Some(format!(
                "Lean meter: post-replace file {projected} LOC > absolute {abs} (split by responsibility or KLEOS_LEAN_FILE_LOC_MAX)"
            ));
        }
        let delta = line_count(n) as isize - line_count(o) as isize;
        let lim = lim_delta(pol) as isize;
        if delta > lim {
            return Some(format!(
                "Lean meter: StrReplace net +{delta} LOC > {lim} (reuse/split or KLEOS_LEAN=0 / KLEOS_LEAN_NET_DELTA)"
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pol() -> LeanPolicy {
        LeanPolicy {
            enabled_env: "KLEOS_LEAN_T_UNSET".into(),
            enabled_default: true,
            new_file_loc_env: "KLEOS_LEAN_NF_T_UNSET".into(),
            new_file_loc: 120,
            net_delta_env: "KLEOS_LEAN_ND_T_UNSET".into(),
            net_delta: 200,
            file_loc_max_env: "KLEOS_LEAN_FM_T_UNSET".into(),
            file_loc_max: 700,
            code_extensions: vec![".ts".into()],
            enforce_always: false,
        }
    }

    fn body(n: usize) -> String {
        (0..n)
            .map(|i| format!("const v{i} = 0;"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn new_file_roof_denies_one_shot() {
        assert!(check("src/a.ts", Some(&body(121)), None, None, &pol()).is_some());
    }

    #[test]
    fn under_roof_allows() {
        assert!(check("src/a.ts", Some(&body(119)), None, None, &pol()).is_none());
    }

    #[test]
    fn non_code_extension_skipped() {
        assert!(check("notes.md", Some(&body(5000)), None, None, &pol()).is_none());
    }

    #[test]
    fn staircase_composition_cannot_exceed_absolute_roof() {
        let dir = std::env::temp_dir().join(format!("kleos_p16_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let f = dir.join("x.ts");
        let path = f.to_string_lossy().to_string();
        let mut size = 1usize;
        fs::write(&f, body(size)).unwrap();
        for _ in 0..20 {
            let next = size + 200;
            if check(&path, Some(&body(next)), None, None, &pol()).is_some() {
                break;
            }
            fs::write(&f, body(next)).unwrap();
            size = next;
        }
        let _ = fs::remove_file(&f);
        assert!(
            size <= 700,
            "P*-16 regression: staircase reached {size} LOC above absolute roof 700"
        );
    }

    #[test]
    fn strreplace_projected_state_bounded() {
        let dir = std::env::temp_dir().join(format!("kleos_p16sr_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let f = dir.join("y.ts");
        fs::write(&f, body(690)).unwrap();
        let path = f.to_string_lossy().to_string();
        let r = check(&path, None, Some(&body(1)), Some(&body(50)), &pol());
        let _ = fs::remove_file(&f);
        assert!(r.is_some(), "690 - 1 + 50 LOC must breach absolute roof 700");
    }
}
