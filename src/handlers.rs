use crate::types::{DbLogEntry, LogEntry, ErrorMessage};
use crate::db::{Db};
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};
use warp::reply::WithStatus;
use warp::reply::Json;
use warp::http::StatusCode;

pub async fn create_log_entry(mut log_entry: LogEntry, db: Db) -> Result<WithStatus<Json>, Infallible> {
    let micros = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    log_entry.timestamp = Some(micros as i64);

    let db_log_entry = DbLogEntry {
        timestamp: micros as i64,
        level: log_entry.level.clone() as i8,
        message: log_entry.message.clone()
    };

    let result = async {
        let db = db.lock().await;
        let mut insert = db.insert("entries")?;
        insert.write(&db_log_entry).await?;
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

pub async fn list_log_entries(db: Db) -> Result<WithStatus<Json>, Infallible> {
    let db_log_entries = async {
        let db = db.lock().await;
        let entries = db.query("SELECT ?fields FROM entries").fetch_all::<DbLogEntry>().await?;
        Ok::<Vec<DbLogEntry>, clickhouse::error::Error>(entries)
    }.await;

    match db_log_entries {
        Ok(db_log_entries) => {
            let log_entries: Vec<LogEntry> = db_log_entries.into_iter().map(|row| LogEntry {
                timestamp: Some(row.timestamp),
                level: num::FromPrimitive::from_i8(row.level).unwrap(),
                message: row.message
            }).collect();

            Ok(warp::reply::with_status(
                warp::reply::json(&log_entries),
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