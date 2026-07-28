use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct VernFields {
    pub file_name_pattern: String,
    pub allowed_kinds: Vec<String>,
    pub allowed_path_prefixes: Vec<String>,
    pub forbidden_class_suffixes: Vec<String>,
    pub class_pattern: String,
    pub function_pattern: String,
    pub boolean_prefixes: Vec<String>,
    pub constant_pattern: String,
}

fn parse_list(val: &str) -> Vec<String> {
    if matches!(val, "[]" | "none" | "-" | "") {
        return Vec::new();
    }
    val.split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn parse_fields(text: &str) -> VernFields {
    let mut f = VernFields::default();
    let re = Regex::new(
        r"(?mi)^(file_name_pattern|allowed_kinds|allowed_path_prefixes|forbidden_class_suffixes|class_pattern|function_pattern|boolean_prefixes|constant_pattern|no_prose_comments|machine_directives_only)\s*:\s*(.+)$",
    )
    .expect("vern field re");
    for cap in re.captures_iter(text) {
        let key = cap[1].to_lowercase();
        let val = cap[2].trim().trim_matches('`').trim().to_string();
        if val.eq_ignore_ascii_case("TBD") || val.is_empty() {
            continue;
        }
        match key.as_str() {
            "allowed_kinds" => f.allowed_kinds = parse_list(&val),
            "allowed_path_prefixes" => f.allowed_path_prefixes = parse_list(&val),
            "forbidden_class_suffixes" => f.forbidden_class_suffixes = parse_list(&val),
            "boolean_prefixes" => f.boolean_prefixes = parse_list(&val),
            "file_name_pattern" => f.file_name_pattern = val,
            "class_pattern" => f.class_pattern = val,
            "function_pattern" => f.function_pattern = val,
            "constant_pattern" => f.constant_pattern = val,
            _ => {}
        }
    }
    f
}

pub fn find_contract(start: &Path) -> Option<PathBuf> {
    let abs = if start.is_absolute() {
        start.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(start)
    };
    let mut cur = if abs.is_dir() {
        abs
    } else {
        abs.parent()?.to_path_buf()
    };
    for _round in 0..32 {
        let candidates = [
            cur.join(".cursor/rules/vernacular.mdc"),
            cur.join("VERNACULAR.md"),
            cur.join("docs/VERNACULAR.md"),
            cur.join("project-rules/vernacular.mdc"),
        ];
        for c in candidates {
            if c.is_file() {
                return Some(c);
            }
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

pub fn path_allowed(path: &Path, contract: &Path, fields: &VernFields) -> Result<(), String> {
    if fields.allowed_path_prefixes.is_empty() {
        return Ok(());
    }
    let root = contract_root(contract);
    let rel_s = match path_rel_to_root(path, &root) {
        Some(s) => s,
        None => {
            return Err("path outside vernacular contract root".into());
        }
    };
    for pref in &fields.allowed_path_prefixes {
        let p = pref.replace('\\', "/").trim_start_matches("./").to_string();
        if p.is_empty() {
            continue;
        }
        if rel_s == p.trim_end_matches('/') {
            return Ok(());
        }
        let prefix = if p.ends_with('/') {
            p.clone()
        } else {
            format!("{p}/")
        };
        if rel_s.starts_with(&prefix) {
            return Ok(());
        }
    }
    Err(format!("path {rel_s} outside allowed_path_prefixes"))
}

fn path_rel_to_root(path: &Path, root: &Path) -> Option<String> {
    let root_abs = root.canonicalize().ok()?;
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };
    if let Ok(c) = abs.canonicalize() {
        return c
            .strip_prefix(&root_abs)
            .ok()
            .map(|r| r.to_string_lossy().replace('\\', "/"));
    }
    let root_s = root_abs.to_string_lossy().replace('\\', "/");
    let abs_s = abs.to_string_lossy().replace('\\', "/");
    let abs_s = abs_s.trim_start_matches("./").to_string();
    if let Some(rest) = abs_s.strip_prefix(&root_s) {
        return Some(rest.trim_start_matches('/').to_string());
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(cabs) = cwd.canonicalize() {
            if let Ok(rel_cwd) = cabs.strip_prefix(&root_abs) {
                let joined = rel_cwd.join(path);
                return Some(joined.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    Some(path.to_string_lossy().replace('\\', "/"))
}

fn pack_native_ok(name: &str) -> bool {
    if name == "__init__.py" || name == "__main__.py" {
        return true;
    }
    let lower = name.to_lowercase();
    if lower == "index.tsx"
        || lower == "index.jsx"
        || lower == "main.tsx"
        || lower == "main.jsx"
        || lower == "app.tsx"
        || lower == "app.jsx"
    {
        return true;
    }
    if is_component_ext(&lower) {
        let base = name.split('.').next().unwrap_or("");
        if Regex::new(r"^[A-Z][A-Za-z0-9]*$")
            .ok()
            .map(|re| re.is_match(base))
            .unwrap_or(false)
        {
            return true;
        }
    }
    if name.starts_with('_') {
        return Regex::new(r"^_[a-z][a-z0-9_-]*(\.[a-z0-9_-]+)+$")
            .ok()
            .map(|re| re.is_match(name))
            .unwrap_or(false);
    }
    Regex::new(r"^[a-z][a-z0-9_-]*(\.[a-z0-9_-]+)+$")
        .ok()
        .map(|re| re.is_match(name))
        .unwrap_or(true)
}

fn is_component_ext(lower_name: &str) -> bool {
    lower_name.ends_with(".tsx")
        || lower_name.ends_with(".jsx")
        || lower_name.ends_with(".vue")
        || lower_name.ends_with(".svelte")
}

fn snake_functions_apply(path: &str) -> bool {
    let lower = path.to_lowercase();
    !is_component_ext(&lower)
        && !lower.ends_with(".css")
        && !lower.ends_with(".scss")
        && !lower.ends_with(".sass")
        && !lower.ends_with(".less")
        && !lower.ends_with(".html")
        && !lower.ends_with(".htm")
}

pub fn file_name_ok(name: &str, fields: &VernFields) -> bool {
    let pat = fields.file_name_pattern.as_str();
    if pat.is_empty() || pat == "kebab_or_snake" || pat == "free" {
        return true;
    }
    if pat == "pack_native" {
        return pack_native_ok(name);
    }
    true
}

fn contract_root(contract: &Path) -> PathBuf {
    if contract.file_name().and_then(|s| s.to_str()) == Some("vernacular.mdc") {
        if let Some(parent) = contract.parent() {
            let pname = parent.file_name().and_then(|s| s.to_str());
            if pname == Some("project-rules") {
                return parent
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| parent.to_path_buf());
            }
            if pname == Some("rules") {
                if let Some(cursor) = parent.parent() {
                    if cursor.file_name().and_then(|s| s.to_str()) == Some(".cursor") {
                        if let Some(root) = cursor.parent() {
                            return root.to_path_buf();
                        }
                    }
                }
            }
        }
    }
    contract
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn check_body_and_path(path: &str, body: &str) -> Result<(), String> {
    let p = Path::new(path);
    let start = if p.exists() {
        p
    } else {
        p.parent().unwrap_or(p)
    };
    let contract = match find_contract(start) {
        Some(c) => c,
        None => return Ok(()),
    };
    let text = fs::read_to_string(&contract).unwrap_or_default();
    let fields = parse_fields(&text);
    if fields.file_name_pattern.is_empty()
        && fields.function_pattern.is_empty()
        && fields.allowed_path_prefixes.is_empty()
    {
        return Ok(());
    }
    path_allowed(p, &contract, &fields)?;
    let name = p.file_name().and_then(|s| s.to_str()).unwrap_or(path);
    if !file_name_ok(name, &fields) {
        return Err(format!("Blocked vernacular file-name drift: {name}"));
    }
    if body.is_empty() {
        return Ok(());
    }
    if fields.function_pattern == "snake_case" && snake_functions_apply(path) {
        let re = Regex::new(
            r"(?:^|\n)\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)|(?:^|\n)\s*def\s+([A-Za-z_][A-Za-z0-9_]*)|(?:^|\n)\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_][A-Za-z0-9_]*)\s*=",
        )
        .expect("fn re");
        let snake = Regex::new(r"^_?[a-z][a-z0-9_]*$").expect("snake");
        for cap in re.captures_iter(body) {
            let fname = cap
                .get(1)
                .or_else(|| cap.get(2))
                .or_else(|| cap.get(3))
                .map(|m| m.as_str())
                .unwrap_or("");
            if fname.is_empty() {
                continue;
            }
            if !snake.is_match(fname) {
                return Err(format!(
                    "Blocked vernacular naming drift: name {fname} violates snake_case"
                ));
            }
        }
    }
    if fields.class_pattern == "PascalCase" && !fields.forbidden_class_suffixes.is_empty() {
        let cre = Regex::new(r"(?:^|[^A-Za-z0-9_-])class\s+([A-Z][A-Za-z0-9_]*)").expect("class");
        for cap in cre.captures_iter(body) {
            let cname = &cap[1];
            for sfx in &fields.forbidden_class_suffixes {
                if cname.ends_with(sfx) {
                    return Err(format!(
                        "Blocked vernacular naming drift: class {cname} forbidden suffix {sfx}"
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_project_rules_vernacular() {
        let root = std::env::temp_dir().join(format!(
            "kleos_vern_{}",
            std::process::id()
        ));
        let ignored = fs::remove_dir_all(&root);
        drop(ignored);
        fs::create_dir_all(root.join("project-rules")).unwrap();
        fs::create_dir_all(root.join("hooks/kleos-gate/src")).unwrap();
        fs::write(
            root.join("project-rules/vernacular.mdc"),
            "file_name_pattern: pack_native\nallowed_path_prefixes: hooks/\n",
        )
        .unwrap();
        let start = root.join("hooks/kleos-gate/src");
        let found = find_contract(&start).expect("contract");
        assert!(
            found.ends_with("project-rules/vernacular.mdc"),
            "{found:?}"
        );
        let bad = root.join("src/FooUseCase.rs");
        let err = check_body_and_path(bad.to_str().unwrap(), "pub struct X {}\n");
        assert!(err.is_err(), "{err:?}");
        let ignored2 = fs::remove_dir_all(&root);
        drop(ignored2);
    }

    #[test]
    fn pack_native_allows_react_pascal_tsx() {
        let f = VernFields {
            file_name_pattern: "pack_native".into(),
            ..VernFields::default()
        };
        assert!(file_name_ok("Button.tsx", &f));
        assert!(file_name_ok("App.tsx", &f));
        assert!(file_name_ok("main.tsx", &f));
        assert!(file_name_ok("hub.home.page.tsx", &f));
        assert!(!file_name_ok("FooUseCase.rs", &f));
        assert!(!file_name_ok("Bad Name.tsx", &f));
    }

    #[test]
    fn snake_functions_skip_tsx() {
        assert!(!snake_functions_apply("src/Button.tsx"));
        assert!(snake_functions_apply("hooks/kleos-gate/src/engine/shell.rs"));
    }
}
