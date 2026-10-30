use crate::error::Result;
use crate::log;

const SERVICE: &str = "envctl";

fn entry(key: &str) -> Result<keyring::Entry> {
    keyring::Entry::new(SERVICE, key)
        .map_err(|e| crate::error::EnvctlError::Keyring(format!("entry {key}: {e}")))
}

pub fn secret_set(key: &str, value: &str) -> Result<()> {
    let e = entry(key)?;
    e.set_password(value)
        .map_err(|e| crate::error::EnvctlError::Keyring(format!("set {key}: {e}")))?;
    log(&format!("stored secret {key} in keyring"));
    Ok(())
}

pub fn secret_get(key: &str) -> Result<Option<String>> {
    let e = entry(key)?;
    match e.get_password() {
        Ok(v) => Ok(Some(v)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(crate::error::EnvctlError::Keyring(format!(
            "get {key}: {e}"
        ))),
    }
}

pub fn secret_remove(key: &str) -> Result<()> {
    let e = entry(key)?;
    match e.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(crate::error::EnvctlError::Keyring(format!(
            "remove {key}: {e}"
        ))),
    }
}

pub fn resolve_value(value: &str) -> Result<String> {
    let prefix = "secret://keyring/";
    if value.starts_with(prefix) {
        let name = &value[prefix.len()..];
        match secret_get(name)? {
            Some(v) => Ok(v),
            None => Err(crate::error::EnvctlError::Keyring(format!(
                "secret ref '{name}' not found in keyring"
            ))),
        }
    } else {
        Ok(value.to_string())
    }
}

pub fn is_secret_ref(value: &str) -> bool {
    value.starts_with("secret://keyring/")
}
