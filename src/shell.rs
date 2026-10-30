use crate::config::Locations;
use crate::error::Result;
use crate::log;
use crate::model::Environment;
use libc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn detect() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_default();
        let base = std::path::Path::new(&shell)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("sh");
        match base {
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            _ => Shell::Bash,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
        }
    }
}

/// Quote a value as a single shell word (data, never code).
pub fn quote(shell: Shell, value: &str) -> String {
    match shell {
        Shell::Bash | Shell::Zsh => {
            // Single quotes preserve everything literally; escape embedded '.
            let esc = value.replace('\'', "'\\''");
            format!("'{esc}'")
        }
        Shell::Fish => {
            // Double quotes: escape backslash, double-quote, dollar, backtick, bang.
            let esc = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('$', "\\$")
                .replace('`', "\\`")
                .replace('!', "\\!");
            format!("\"{esc}\"")
        }
    }
}

/// Quote a bare word (e.g. alias name, function name) — assumed safe identifier.
pub fn word(value: &str) -> String {
    value.to_string()
}

/// Saved activation state, keyed by the shell's PPID.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivationState {
    pub ppid: i32,
    pub active: Option<String>,
    pub saved_vars: BTreeMap<String, Option<String>>,
    pub saved_path: Option<String>,
    pub saved_cwd: Option<String>,
}

/// Compute the state file path for the current shell (keyed by PPID).
fn state_file(locations: &Locations, ppid: i32) -> PathBuf {
    locations.active_dir().join(format!("{ppid}"))
}

/// Emit `activate` shell code. Also persists the pre-activation snapshot.
pub fn emit_activate(
    locations: &Locations,
    shell: Shell,
    env: &Environment,
    no_functions: bool,
    no_aliases: bool,
    no_cwd: bool,
) -> Result<String> {
    let ppid = unsafe { libc::getppid() };
    let mut state = ActivationState {
        ppid,
        ..ActivationState::default()
    };
    state.active = Some(env.name.clone());

    // Snapshot current values of keys we are about to override.
    for key in env.vars.keys() {
        state
            .saved_vars
            .insert(key.clone(), std::env::var(key).ok());
    }
    state.saved_path = std::env::var("PATH").ok();
    if env.cwd.is_some() {
        state.saved_cwd = std::env::current_dir()
            .ok()
            .map(|p| p.to_string_lossy().into_owned());
    }

    // Persist snapshot so deactivate can restore.
    fs::create_dir_all(locations.active_dir())?;
    let sf = state_file(locations, ppid);
    let data = serde_json::to_string_pretty(&state)?;
    let mut f = fs::File::create(&sf)?;
    use std::io::Write;
    f.write_all(data.as_bytes())?;
    f.sync_all()?;
    log(&format!("saved activation state for ppid {ppid}"));

    let mut lines: Vec<String> = vec![];
    lines.push(format!("# envctl activate {}", env.name));
    lines.push(format!("export ENVCTL_ACTIVE={}", quote(shell, &env.name)));

    // Vars.
    for (k, v) in &env.vars {
        let resolved = crate::secrets::resolve_value(v).unwrap_or_else(|_| v.clone());
        lines.push(format!("export {k}={}", quote(shell, &resolved)));
    }

    // PATH.
    if !env.paths.prepend.is_empty() {
        let joined = env.paths.prepend.join(":");
        lines.push(format!("export PATH={}:$PATH", quote(shell, &joined)));
    }
    if !env.paths.append.is_empty() {
        let joined = env.paths.append.join(":");
        lines.push(format!("export PATH=$PATH:{}", quote(shell, &joined)));
    }

    // Aliases.
    if !no_aliases {
        for (k, v) in &env.aliases {
            match shell {
                Shell::Bash | Shell::Zsh => lines.push(format!("alias {k}={}", quote(shell, v))),
                Shell::Fish => lines.push(format!("alias {k} {}", quote(shell, v))),
            }
        }
    }

    // Functions (user-supplied code, emitted verbatim; guarded by flag).
    if !no_functions {
        for (k, body) in &env.functions {
            match shell {
                Shell::Bash | Shell::Zsh => {
                    lines.push(format!("{k}() {{{body}}}"));
                }
                Shell::Fish => {
                    lines.push(format!("function {k}; {body}; end"));
                }
            }
        }
    }

    // CWD.
    if !no_cwd {
        if let Some(cwd) = &env.cwd {
            match shell {
                Shell::Bash | Shell::Zsh => lines.push(format!("cd {}", quote(shell, &cwd.path))),
                Shell::Fish => lines.push(format!("cd {}", quote(shell, &cwd.path))),
            }
        }
    }

    Ok(lines.join("\n") + "\n")
}

/// Emit `deactivate` shell code restoring the pre-activation snapshot.
pub fn emit_deactivate(locations: &Locations, shell: Shell) -> Result<String> {
    let ppid = unsafe { libc::getppid() };
    let sf = state_file(locations, ppid);
    if !sf.exists() {
        return Err(crate::error::EnvctlError::NotFound(
            "no active environment to deactivate".into(),
        ));
    }
    let data = fs::read_to_string(&sf)?;
    let state: ActivationState =
        serde_json::from_str(&data).map_err(|e| crate::error::EnvctlError::Parse(e.to_string()))?;

    let mut lines: Vec<String> = vec![];
    lines.push("# envctl deactivate".to_string());

    for (k, v) in &state.saved_vars {
        match v {
            Some(val) => lines.push(format!("export {k}={}", quote(shell, val))),
            None => lines.push(format!("unset {k}")),
        }
    }
    if let Some(p) = &state.saved_path {
        lines.push(format!("export PATH={}", quote(shell, p)));
    }
    if let Some(cwd) = &state.saved_cwd {
        match shell {
            Shell::Bash | Shell::Zsh => lines.push(format!("cd {}", quote(shell, cwd))),
            Shell::Fish => lines.push(format!("cd {}", quote(shell, cwd))),
        }
    }
    lines.push("unset ENVCTL_ACTIVE".to_string());

    // Remove state file.
    let _ = fs::remove_file(&sf);

    Ok(lines.join("\n") + "\n")
}

/// Print the active environment name for the current shell.
pub fn active_name(locations: &Locations) -> Result<Option<String>> {
    // Prefer the exported env var (set by activate), fall back to state file.
    if let Ok(v) = std::env::var("ENVCTL_ACTIVE") {
        if !v.is_empty() {
            return Ok(Some(v));
        }
    }
    let ppid = unsafe { libc::getppid() };
    let sf = state_file(locations, ppid);
    if !sf.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(&sf)?;
    let state: ActivationState =
        serde_json::from_str(&data).map_err(|e| crate::error::EnvctlError::Parse(e.to_string()))?;
    Ok(state.active)
}
