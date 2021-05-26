use clickhouse::Reflection;
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, Reflection)]
#[repr(u8)]
pub enum LogLevelInternal {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflection)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

impl From<LogLevelInternal> for LogLevel {
    fn from(item: LogLevelInternal) -> Self {
        match item {
            LogLevelInternal::DEBUG => LogLevel::DEBUG,
            LogLevelInternal::INFO => LogLevel::INFO,
            LogLevelInternal::WARNING => LogLevel::WARNING,
            LogLevelInternal::ERROR => LogLevel::ERROR
        }
    }
}

impl From<LogLevel> for LogLevelInternal {
    fn from(item: LogLevel) -> Self {
        match item {
            LogLevel::DEBUG => LogLevelInternal::DEBUG,
            LogLevel::INFO => LogLevelInternal::INFO,
            LogLevel::WARNING => LogLevelInternal::WARNING,
            LogLevel::ERROR => LogLevelInternal::ERROR
        }
    }
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevelInternal,
    pub message: String
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntryInput {
    pub level: LogLevel,
    pub message: String
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntryOutput {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String
}

#[derive(Debug, Deserialize)]
pub struct LogViewQuery {
    pub level: LogLevel,
    pub timestamp_ge: u64,
    pub timestamp_le: u64
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}