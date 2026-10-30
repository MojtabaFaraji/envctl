use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Environment {
    #[serde(default = "default_version")]
    pub version: u32,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub vars: BTreeMap<String, String>,
    #[serde(default)]
    pub paths: Paths,
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
    #[serde(default)]
    pub functions: BTreeMap<String, String>,
    #[serde(default)]
    pub cwd: Option<Cwd>,
}

fn default_version() -> u32 {
    SCHEMA_VERSION
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paths {
    #[serde(default)]
    pub prepend: Vec<String>,
    #[serde(default)]
    pub append: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cwd {
    pub path: String,
}

impl Environment {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            version: SCHEMA_VERSION,
            name: name.clone(),
            description: String::new(),
            tags: vec![],
            created_at: Some(now),
            updated_at: Some(now),
            vars: BTreeMap::new(),
            paths: Paths::default(),
            aliases: BTreeMap::new(),
            functions: BTreeMap::new(),
            cwd: None,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}

/// Validate an environment name.
pub fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("name must not be empty".into());
    }
    if name.len() > 64 {
        return Err("name too long (max 64)".into());
    }
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphanumeric() {
        return Err("name must start with alphanumeric".into());
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-') {
            return Err(format!("invalid character in name: {c}"));
        }
    }
    if name.contains("..") {
        return Err("name must not contain '..'".into());
    }
    Ok(())
}

/// Validate a var key.
pub fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("key must not be empty".into());
    }
    let mut chars = key.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err("key must start with letter or underscore".into());
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!("invalid character in key: {c}"));
        }
    }
    Ok(())
}

/// Heuristic: does this key name look like a secret?
pub fn looks_like_secret(key: &str) -> bool {
    let u = key.to_uppercase();
    u.contains("SECRET")
        || u.contains("PASSWORD")
        || u.contains("TOKEN")
        || u.ends_with("_KEY")
        || u.contains("API_KEY")
        || u.contains("PRIVATE")
}