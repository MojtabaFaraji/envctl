use crate::config::Locations;
use crate::error::{EnvctlError, Result};
use crate::log;
use crate::log_debug;
use crate::model::Environment;
use fs2::FileExt;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::time::SystemTime;

/// Persistent store for environments.
pub struct Store {
    pub locations: Locations,
}

impl Store {
    pub fn new(locations: Locations) -> Self {
        Self { locations }
    }

    pub fn init(&self) -> Result<Environment> {
        self.locations.ensure_dirs()?;
        // Write default config if absent.
        let cfg_path = self.locations.config_file();
        if !cfg_path.exists() {
            let default_cfg = r#"# envctl global configuration
default_shell = ""
color = true
editor = ""
allow_plaintext_secrets = false
system = false
"#;
            let tmp = self.atomic_write_tmp(&cfg_path, default_cfg.as_bytes())?;
            self.atomic_rename(&tmp, &cfg_path)?;
        }
        Ok(Environment::new("placeholder".into()))
    }

    /// List all environment names.
    pub fn list_names(&self) -> Result<Vec<String>> {
        let envs_dir = &self.locations.envs_dir;
        if !envs_dir.exists() {
            return Ok(vec![]);
        }
        let mut names = vec![];
        for entry in fs::read_dir(envs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "toml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }
        names.sort();
        Ok(names)
    }

    /// Load one environment by name.
    pub fn load(&self, name: &str) -> Result<Environment> {
        let path = self.locations.env_file(name);
        if !path.exists() {
            return Err(EnvctlError::NotFound(format!(
                "environment '{name}' not found"
            )));
        }
        let s = fs::read_to_string(&path)?;
        let mut env: Environment =
            toml::from_str(&s).map_err(|e| EnvctlError::Parse(format!("{name}: {e}")))?;
        env.name = name.to_string();
        self.migrate(&mut env)?;
        Ok(env)
    }

    /// Save one environment (atomic, locked).
    pub fn save(&self, env: &Environment) -> Result<()> {
        let _lock = self.lock()?;
        let path = self.locations.env_file(&env.name);
        let mut data = toml::to_string_pretty(env)
            .map_err(|e| EnvctlError::Generic(format!("serialize: {e}")))?;
        data.push('\n');
        let tmp = self.atomic_write_tmp(&path, data.as_bytes())?;
        self.set_perms(&tmp)?;
        self.atomic_rename(&tmp, &path)?;
        log(&format!("saved environment {}", env.name));
        Ok(())
    }

    /// Delete one environment by name.
    pub fn delete(&self, name: &str, backup: bool) -> Result<()> {
        let _lock = self.lock()?;
        let path = self.locations.env_file(name);
        if !path.exists() {
            return Err(EnvctlError::NotFound(format!(
                "environment '{name}' not found"
            )));
        }
        if backup {
            self.backup(name)?;
        }
        fs::remove_file(&path)?;
        log(&format!("deleted environment {name}"));
        Ok(())
    }

    /// Exists?
    pub fn exists(&self, name: &str) -> bool {
        self.locations.env_file(name).exists()
    }

    // ---- internal helpers ----

    fn lock(&self) -> Result<fs::File> {
        fs::create_dir_all(&self.locations.state_dir)?;
        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.locations.lock_file())?;
        f.lock_exclusive()
            .map_err(|e| EnvctlError::Io(format!("lock: {e}")))?;
        log_debug("acquired store lock");
        Ok(f)
    }

    fn atomic_write_tmp(&self, target: &Path, data: &[u8]) -> Result<PathBuf> {
        let dir = target.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(dir)?;
        let tmp = dir.join(format!(
            ".{}.{}.tmp",
            target.file_name().and_then(|s| s.to_str()).unwrap_or("env"),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data)?;
        f.flush()?;
        f.sync_all()?;
        log_debug(&format!("wrote temp {}", tmp.display()));
        Ok(tmp)
    }

    fn atomic_rename(&self, tmp: &Path, target: &Path) -> Result<()> {
        fs::rename(tmp, target).map_err(|e| EnvctlError::Io(format!("rename: {e}")))?;
        log_debug(&format!(
            "renamed {} -> {}",
            tmp.display(),
            target.display()
        ));
        Ok(())
    }

    fn set_perms(&self, path: &Path) -> Result<()> {
        let perm = if self.locations.system { 0o640 } else { 0o600 };
        #[allow(unused_mut)]
        {
            fs::set_permissions(path, fs::Permissions::from_mode(perm))?;
        }
        Ok(())
    }

    fn backup(&self, name: &str) -> Result<PathBuf> {
        fs::create_dir_all(&self.locations.backups_dir())?;
        let src = self.locations.env_file(name);
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let dst = self
            .locations
            .backups_dir()
            .join(format!("{name}.{ts}.toml"));
        fs::copy(&src, &dst)?;
        log(&format!("backed up {name} to {}", dst.display()));
        Ok(dst)
    }

    fn migrate(&self, env: &mut Environment) -> Result<()> {
        if env.version == 0 || env.version < crate::model::SCHEMA_VERSION {
            // bump to current
            env.version = crate::model::SCHEMA_VERSION;
        }
        Ok(())
    }
}
