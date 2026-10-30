use crate::error::Result;
use crate::model::Environment;
use serde::Serialize;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Yaml,
    Plain,
}

pub fn render_env(env: &Environment, fmt: OutputFormat) -> Result<String> {
    match fmt {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(env)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(env)?),
        OutputFormat::Plain => Ok(plain_env(env)),
        OutputFormat::Table => Ok(table_env(env)),
    }
}

fn plain_env(env: &Environment) -> String {
    let mut s = String::new();
    writeln!(s, "name: {}", env.name).ok();
    writeln!(s, "description: {}", env.description).ok();
    for (k, v) in &env.vars {
        writeln!(s, "{k}={v}").ok();
    }
    for p in &env.paths.prepend {
        writeln!(s, "PATH+PREPEND={p}").ok();
    }
    for p in &env.paths.append {
        writeln!(s, "PATH+APPEND={p}").ok();
    }
    for (k, v) in &env.aliases {
        writeln!(s, "alias {k}={v}").ok();
    }
    s
}

fn table_env(env: &Environment) -> String {
    let mut s = String::new();
    writeln!(s, "Environment: {}", env.name).ok();
    if !env.description.is_empty() {
        writeln!(s, "  description: {}", env.description).ok();
    }
    if !env.vars.is_empty() {
        writeln!(s, "  vars:").ok();
        for (k, v) in &env.vars {
            let disp = if crate::secrets::is_secret_ref(v) {
                format!("{} (keyring ref)", v)
            } else {
                v.clone()
            };
            writeln!(s, "    {k} = {disp}").ok();
        }
    }
    if !env.paths.prepend.is_empty() {
        writeln!(s, "  path prepend: {}", env.paths.prepend.join(" ")).ok();
    }
    if !env.paths.append.is_empty() {
        writeln!(s, "  path append: {}", env.paths.append.join(" ")).ok();
    }
    if !env.aliases.is_empty() {
        writeln!(s, "  aliases:").ok();
        for (k, v) in &env.aliases {
            writeln!(s, "    {k} = {v}").ok();
        }
    }
    if !env.functions.is_empty() {
        writeln!(
            s,
            "  functions: {}",
            env.functions.keys().cloned().collect::<Vec<_>>().join(", ")
        )
        .ok();
    }
    if let Some(cwd) = &env.cwd {
        writeln!(s, "  cwd: {}", cwd.path).ok();
    }
    s
}

/// Render a list of (name, updated) rows.
pub fn render_list(rows: &[(String, Option<String>)], fmt: OutputFormat) -> Result<String> {
    match fmt {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct Row {
                name: String,
                updated: Option<String>,
            }
            let v: Vec<Row> = rows
                .iter()
                .map(|(n, u)| Row {
                    name: n.clone(),
                    updated: u.clone(),
                })
                .collect();
            Ok(serde_json::to_string_pretty(&v)?)
        }
        OutputFormat::Yaml => Ok(serde_yaml::to_string(rows)?),
        OutputFormat::Plain => {
            let mut s = String::new();
            for (n, _) in rows {
                writeln!(s, "{n}").ok();
            }
            Ok(s)
        }
        OutputFormat::Table => {
            let mut s = String::new();
            writeln!(s, "{:<30} UPDATED", "NAME").ok();
            for (n, u) in rows {
                writeln!(s, "{:<30} {}", n, u.as_deref().unwrap_or("-")).ok();
            }
            Ok(s)
        }
    }
}
