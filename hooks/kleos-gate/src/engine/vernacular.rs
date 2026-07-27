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
    let mut cur = if start.is_dir() {
        start.to_path_buf()
    } else {
        start.parent()?.to_path_buf()
    };
    for _round in 0..32 {
        let a = cur.join(".cursor/rules/vernacular.mdc");
        let b = cur.join("VERNACULAR.md");
        let c = cur.join("docs/VERNACULAR.md");
        if a.is_file() {
            return Some(a);
        }
        if b.is_file() {
            return Some(b);
        }
        if c.is_file() {
            return Some(c);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

fn pack_native_ok(name: &str) -> bool {
    if name == "__init__.py" || name == "__main__.py" {
        return true;
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

pub fn path_allowed(path: &Path, contract: &Path, fields: &VernFields) -> Result<(), String> {
    if fields.allowed_path_prefixes.is_empty() {
        return Ok(());
    }
    let root = contract_root(contract);
    let rel = match path.canonicalize().ok().and_then(|p| {
        root.canonicalize()
            .ok()
            .and_then(|r| p.strip_prefix(r).ok().map(|x| x.to_path_buf()))
    }) {
        Some(r) => r,
        None => return Ok(()),
    };
    let rel_s = rel.to_string_lossy().replace('\\', "/");
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

fn contract_root(contract: &Path) -> PathBuf {
    if contract.file_name().and_then(|s| s.to_str()) == Some("vernacular.mdc") {
        if let Some(rules) = contract.parent() {
            if rules.file_name().and_then(|s| s.to_str()) == Some("rules") {
                if let Some(cursor) = rules.parent() {
                    if let Some(root) = cursor.parent() {
                        return root.to_path_buf();
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
    if fields.function_pattern == "snake_case" {
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
