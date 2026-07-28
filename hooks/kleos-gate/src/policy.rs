use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub pattern: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ShellPolicy {
    pub deny: Vec<Rule>,
    pub ask: Vec<Rule>,
    pub opaque_write_ask_message: String,
    #[serde(default)]
    pub opaque_write_deny_message: String,
    pub prose_shell_deny_message: String,
}

#[derive(Debug, Deserialize)]
pub struct LeanPolicy {
    pub enabled_env: String,
    pub enabled_default: bool,
    pub new_file_loc_env: String,
    pub new_file_loc: usize,
    pub net_delta_env: String,
    pub net_delta: usize,
    pub file_loc_max_env: String,
    pub file_loc_max: usize,
    pub code_extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SecretsPolicy {
    pub content_pattern: String,
    pub content_message: String,
    pub allow_substring: String,
    pub read_path_pattern: String,
    pub read_allow_pattern: String,
    pub read_message: String,
    pub mcp_danger_pattern: String,
    pub mcp_secret_message: String,
    pub mcp_ask_message: String,
}

#[derive(Debug, Deserialize)]
pub struct AskScopePolicy {
    pub enabled: bool,
    pub mode: String,
    pub message: String,
    pub path_token_pattern: String,
    pub exempt_prefixes: Vec<String>,
    pub min_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub struct DeletePolicy {
    pub message: String,
    pub deny_recursive: bool,
    pub deny_multi_path: bool,
    pub deny_globs_and_roots: bool,
    pub deny_extensionless_basename: bool,
    #[serde(default)]
    pub tree_basename_suffixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClassifyRule {
    pub id: String,
    pub pattern: String,
    pub hint: String,
}

#[derive(Debug, Deserialize)]
pub struct ContextPolicy {
    pub vault_root_env: String,
    pub vault_root: String,
    pub hot_path: String,
    pub index_path: String,
    pub hot_chars_max: usize,
    pub pointer_max: usize,
    pub pointer_min_hits: usize,
    pub pointer_token_min_len: usize,
    pub pointer_stopwords: Vec<String>,
    pub recall_gate_enabled: bool,
    pub recall_message: String,
    pub exempt_write_prefixes: Vec<String>,
    pub meter_enabled: bool,
    pub classify_max: usize,
    pub classify_rules: Vec<ClassifyRule>,
    pub playbook: String,
}

#[derive(Debug)]
pub struct Policy {
    pub shell: ShellPolicy,
    pub lean: LeanPolicy,
    pub secrets: SecretsPolicy,
    pub ask_scope: AskScopePolicy,
    pub delete: DeletePolicy,
    pub context: ContextPolicy,
}

impl Policy {
    pub fn load(dir: &Path) -> Result<Self, String> {
        let shell: ShellPolicy = read_json(&dir.join("shell.json"))?;
        let lean: LeanPolicy = read_json(&dir.join("lean.json"))?;
        let secrets: SecretsPolicy = read_json(&dir.join("secrets.json"))?;
        let ask_scope: AskScopePolicy = read_json(&dir.join("ask-scope.json"))?;
        let delete: DeletePolicy = read_json(&dir.join("delete.json"))?;
        let context: ContextPolicy = read_json(&dir.join("context.json"))?;
        Ok(Self {
            shell,
            lean,
            secrets,
            ask_scope,
            delete,
            context,
        })
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))
}
