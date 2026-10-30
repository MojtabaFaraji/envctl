pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod model;
pub mod output;
pub mod secrets;
pub mod shell;
pub mod storage;

pub use error::{EnvctlError, Result};
pub use model::Environment;
pub use storage::Store;

use once_cell::sync::Lazy;

pub static LOG_LEVEL: Lazy<std::sync::atomic::AtomicU8> =
    Lazy::new(|| std::sync::atomic::AtomicU8::new(0));

pub fn set_verbose(verbose: u8) {
    LOG_LEVEL.store(verbose, std::sync::atomic::Ordering::SeqCst);
}

pub fn is_verbose() -> bool {
    LOG_LEVEL.load(std::sync::atomic::Ordering::SeqCst) > 0
}

pub fn log(msg: &str) {
    if is_verbose() {
        eprintln!("[envctl] {msg}");
    }
}

pub fn log_debug(msg: &str) {
    if LOG_LEVEL.load(std::sync::atomic::Ordering::SeqCst) >= 2 {
        eprintln!("[envctl:debug] {msg}");
    }
}

pub fn log_err(msg: &str) {
    eprintln!("error: {msg}");
}
