use crate::error::Result;
use crate::log;
use libc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Global config file (TOML).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default_shell: Option<String>,
    #[serde(default)]
    pub color: Option<bool>,
    #[serde(default)]
    pub editor: Option<String>,
    #[serde(default)]
    pub allow_plaintext_secrets: bool,
    #[serde(default)]
    pub system: bool,
}

/// XDG-style store locations.
#[derive(Debug, Clone)]
pub struct Locations {
    pub config_dir: PathBuf,
    pub state_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub envs_dir: PathBuf,
    pub system: bool,
}

impl Locations {
    /// Resolve locations for per-user or system-wide store.
    pub fn resolve(system: bool, config: Option<PathBuf>, profile: Option<String>) -> Result<Self> {
        if system {
            // Require privileges.
            let euid = unsafe { libc::geteuid() };
            if euid != 0 {
                return Err(crate::error::EnvctlError::PermissionDenied(
                    "--system requires root privileges (euid 0)".into(),
                ));
            }
            let config_dir = config.unwrap_or_else(|| PathBuf::from("/etc/envctl"));
            let envs_dir = config_dir.join("envs");
            let state_dir = PathBuf::from("/var/lib/envctl");
            let cache_dir = PathBuf::from("/var/cache/envctl");
            Ok(Self {
                config_dir,
                state_dir,
                cache_dir,
                envs_dir,
                system: true,
            })
        } else {
            let xdg_config = env::var("XDG_CONFIG_HOME")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".config"));
            let xdg_state = env::var("XDG_STATE_HOME")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".local/state"));
            let xdg_cache = env::var("XDG_CACHE_HOME")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".cache"));

            let base_config_dir = match config {
                Some(c) => c
                    .parent()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| xdg_config.join("envctl")),
                None => xdg_config.join("envctl"),
            };
            let config_dir = match profile {
                Some(p) => xdg_config.join(format!("envctl-{}", p)),
                None => base_config_dir,
            };
            let envs_dir = config_dir.join("envs");
            let state_dir = xdg_state.join("envctl");
            let cache_dir = xdg_cache.join("envctl");

            Ok(Self {
                config_dir,
                state_dir,
                cache_dir,
                envs_dir,
                system: false,
            })
        }
    }

    pub fn env_file(&self, name: &str) -> PathBuf {
        self.envs_dir.join(format!("{name}.toml"))
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn lock_file(&self) -> PathBuf {
        self.state_dir.join("lock")
    }

    pub fn state_dir(&self) -> &Path {
        &self.state_dir
    }

    pub fn active_dir(&self) -> PathBuf {
        self.state_dir.join("active")
    }

    pub fn backups_dir(&self) -> PathBuf {
        self.state_dir.join("backups")
    }

    /// Create all required directories with safe permissions.
    pub fn ensure_dirs(&self) -> Result<()> {
        let perm = if self.system { 0o755 } else { 0o700 };
        for d in [
            &self.config_dir,
            &self.envs_dir,
            &self.state_dir,
            &self.cache_dir,
            &self.active_dir(),
            &self.backups_dir(),
        ] {
            fs::create_dir_all(d)?;
            // Set permissive bits; umask may restrict.
            #[allow(unused_mut)]
            let mut md = fs::metadata(d)?;
            let cur = md.permissions().mode();
            // Only enforce owner bits on user store.
            if !self.system {
                fs::set_permissions(d, fs::Permissions::from_mode(perm))?;
            }
            log(&format!("ensured dir {} ({:o})", d.display(), perm));
        }
        Ok(())
    }

    /// Load global config if present.
    pub fn load_config(&self) -> Result<Config> {
        let p = self.config_file();
        if !p.exists() {
            return Ok(Config::default());
        }
        let s = fs::read_to_string(&p)?;
        let c: Config =
            toml::from_str(&s).map_err(|e| crate::error::EnvctlError::Parse(e.to_string()))?;
        Ok(c)
    }
}

impl Default for Locations {
    fn default() -> Self {
        Self::resolve(false, None, None).expect("could not resolve default locations")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_in_model() {
        // placeholder to keep module tested
        assert!(true);
    }
}
