use crate::types::{LogEntry, LogEntryInput, LogEntryOutput, LogLevelInternal, LogViewQuery, ErrorMessage};
use crate::db::{Db};
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};
use warp::reply::WithStatus;
use warp::reply::Json;
use warp::http::StatusCode;

pub async fn create_log_entry(log_entry_input: LogEntryInput, db: Db) -> Result<WithStatus<Json>, Infallible> {
    let micros = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

    let log_entry = LogEntry {
        timestamp: micros as u64,
        level: log_entry_input.level.into(),
        message: log_entry_input.message
    };

    let result = async {
        let db = db.lock().await;
        let mut insert = db.insert("entries")?;
        insert.write(&log_entry).await?;
        insert.end().await?;
        Ok::<(), clickhouse::error::Error>(())
    }.await;

    match result {
        Ok(_v) => Ok(warp::reply::with_status(
            warp::reply::json(&log_entry),
            StatusCode::CREATED
        )),
        Err(err) => Ok(warp::reply::with_status(
            warp::reply::json(&ErrorMessage { message: err.to_string() }),
            StatusCode::INTERNAL_SERVER_ERROR
        ))
    }
}

pub async fn list_log_entries(query: LogViewQuery, db: Db) -> Result<WithStatus<Json>, Infallible> {
    let level_internal: LogLevelInternal = query.level.clone().into();

    let log_entries = async {
        let db = db.lock().await;
        let entries = db.query("SELECT ?fields FROM entries WHERE level == ? AND timestamp BETWEEN fromUnixTimestamp64Nano(toInt64(?)) AND fromUnixTimestamp64Nano(toInt64(?))")
            .bind(level_internal as u8)
            .bind(query.timestamp_ge)
            .bind(query.timestamp_le)
            .fetch_all::<LogEntry>().await?;
        Ok::<Vec<LogEntry>, clickhouse::error::Error>(entries)
    }.await;

    match log_entries {
        Ok(log_entries) => {
            let log_entries_output: Vec<LogEntryOutput> = log_entries.iter().map(|entry| LogEntryOutput {
                timestamp: entry.timestamp,
                level: entry.level.clone().into(),
                message: entry.message.clone()
            }).collect();
            Ok(warp::reply::with_status(
                warp::reply::json(&log_entries_output),
                StatusCode::OK
            ))
        },
        Err(err) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage { message: err.to_string() }),
                StatusCode::INTERNAL_SERVER_ERROR
            ))
        }
    }
}