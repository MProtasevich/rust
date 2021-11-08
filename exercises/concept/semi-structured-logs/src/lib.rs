// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::fmt::{ Display, Formatter, Result };

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use LogLevel::*;
        match *self {
            Info => write!(f, "INFO"),
            Warning => write!(f, "WARNING"),
            Error => write!(f, "ERROR"),
        }
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
