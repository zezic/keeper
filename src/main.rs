#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};
use clickhouse::Reflection;
use warp::{reject, Filter};
use strum_macros::EnumString;

extern crate num;
#[macro_use]
extern crate num_derive;


#[derive(EnumString, FromPrimitive, Clone, Reflection, Deserialize, Serialize)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntry {
    timestamp: Option<i64>,
    level: LogLevel,
    message: String
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct DbLogEntry {
    timestamp: i64,
    level: i8,
    message: String
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

impl reject::Reject for ErrorMessage {}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = db::get_client();

    let api = filters::logger(db);
    let routes = api.with(warp::log("Log Service"));
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await
}

mod filters {
    use super::db::{Db};
    use super::handlers;
    use warp::Filter;

    pub fn logger(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        log_entry_post(db.clone())
            .or(log_entries_get(db.clone()))
    }

    pub fn log_entry_post(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("log")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_log_entry)
    }

    pub fn log_entries_get(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("entries")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_log_entries)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use super::db::{Db};
    use super::{ErrorMessage, LogEntry, DbLogEntry};
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
}

mod db {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use clickhouse::Client;

    pub type Db = Arc<Mutex<clickhouse::Client>>;

    pub fn get_client() -> Db {
        let client = Client::default()
            // .with_url("http://localhost:8123")
            .with_url("http://clickhouse:8123")
            .with_user("keeper")
            .with_password("12345")
            .with_database("keeper");
        Arc::new(Mutex::new(client))
    }
}