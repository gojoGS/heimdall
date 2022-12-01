
use serde_derive::{Deserialize, Serialize};




mod filter;

#[repr(i8)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LogLevel {
    None = 0,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn from_str(string: &str) -> LogLevel {
        match string.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => LogLevel::None,
        }
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        let level = match self {
            LogLevel::None => "NONE",
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
            _ => "NONE",
        };

        level.to_string()
    }
}

pub struct LogLine {
    pub log_level: LogLevel,
    pub line: String,
}
