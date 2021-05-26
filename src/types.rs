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

#[derive(Clone, Serialize, Deserialize, Reflection)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
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

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}