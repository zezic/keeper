use clickhouse::Reflection;
use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;
extern crate num;

#[derive(EnumString, FromPrimitive, Clone, Reflection, Deserialize, Serialize)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: Option<i64>,
    pub level: LogLevel,
    pub message: String
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct DbLogEntry {
    pub timestamp: i64,
    pub level: i8,
    pub message: String
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}