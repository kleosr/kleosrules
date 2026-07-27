use crate::policy::LeanPolicy;
use std::env;
use std::fs;
use std::path::Path;

pub fn enabled(pol: &LeanPolicy) -> bool {
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

pub fn check(path: &str, contents: Option<&str>, old: Option<&str>, new: Option<&str>, pol: &LeanPolicy) -> Option<String> {
    if !enabled(pol) || !is_code_path(path, pol) {
        return None;
    }
    if let Some(body) = contents {
        let n = line_count(body);
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
