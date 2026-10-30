use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvctlError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("usage error: {0}")]
    Usage(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("validation error:\n{0}")]
    Validation(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("keyring error: {0}")]
    Keyring(String),
    #[error("{0}")]
    Generic(String),
}

impl EnvctlError {
    pub fn exit_code(&self) -> i32 {
        match self {
            EnvctlError::NotFound(_) => 3,
            EnvctlError::PermissionDenied(_) => 4,
            EnvctlError::Usage(_) => 2,
            _ => 1,
        }
    }
}

impl From<io::Error> for EnvctlError {
    fn from(e: io::Error) -> Self {
        if e.kind() == io::ErrorKind::PermissionDenied {
            EnvctlError::PermissionDenied(e.to_string())
        } else if e.kind() == io::ErrorKind::NotFound {
            EnvctlError::NotFound(e.to_string())
        } else {
            EnvctlError::Io(e.to_string())
        }
    }
}

impl From<serde_json::Error> for EnvctlError {
    fn from(e: serde_json::Error) -> Self {
        EnvctlError::Parse(e.to_string())
    }
}
impl From<serde_yaml::Error> for EnvctlError {
    fn from(e: serde_yaml::Error) -> Self {
        EnvctlError::Parse(e.to_string())
    }
}
impl From<toml::de::Error> for EnvctlError {
    fn from(e: toml::de::Error) -> Self {
        EnvctlError::Parse(e.to_string())
    }
}
impl From<toml::ser::Error> for EnvctlError {
    fn from(e: toml::ser::Error) -> Self {
        EnvctlError::Generic(e.to_string())
    }
}

impl From<String> for EnvctlError {
    fn from(e: String) -> Self {
        EnvctlError::Usage(e)
    }
}

pub type Result<T> = std::result::Result<T, EnvctlError>;
