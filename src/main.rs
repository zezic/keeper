#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};
use clickhouse::Reflection;
use warp::Filter;
use strum_macros::EnumString;


#[derive(EnumString, Clone, Reflection, Deserialize, Serialize)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct LogEntry {
    level: LogLevel,
    message: String,
}

#[derive(Clone, Reflection, Deserialize, Serialize)]
pub struct DbLogEntry {
    level: String,
    message: String
}

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
    use super::{LogEntry, LogLevel, DbLogEntry};
    use std::convert::Infallible;
    use std::str::FromStr;

    pub async fn create_log_entry(entry: LogEntry, db: Db) -> Result<impl warp::Reply, Infallible> {
        let db = db.lock().await;

        let db_entry = DbLogEntry {
            level: match entry.level {
                LogLevel::DEBUG => "DEBUG",
                LogLevel::INFO => "INFO",
                LogLevel::WARNING => "WARNING",
                LogLevel::ERROR => "ERROR"
            }.to_string(),
            message: entry.message.clone()
        };

        let result = async {
            let mut insert = db.insert("entries")?;
            insert.write(&db_entry).await?;
            insert.end().await?;
            Ok::<(), clickhouse::error::Error>(())
        }.await;

        match result {
            Ok(_v) => Ok(warp::reply::json(&entry)),
            Err(_err) => Ok(warp::reply::json(&"ClickHouse error"))
        }
    }

    pub async fn list_log_entries(db: Db) -> Result<impl warp::Reply, Infallible> {
        let db = db.lock().await;

        let entries = async {
            let mut vector: Vec<LogEntry> = vec![];
            let mut cursor = db.query("SELECT ?fields FROM entries").fetch::<DbLogEntry>()?;
            while let Some(row) = cursor.next().await? {
                let entry = LogEntry {
                    level: LogLevel::from_str(&row.level).unwrap(),
                    message: row.message
                };
                vector.push(entry);
            }
            Ok::<Vec<LogEntry>, clickhouse::error::Error>(vector)
        }.await;

        match entries {
            Ok(v) => Ok(warp::reply::json(&v)),
            Err(err) => {
                println!("{}", err);
                Ok(warp::reply::json(&"ClickHouse error"))
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
            .with_url("http://clickhouse:8123")
            .with_user("keeper")
            .with_password("12345")
            .with_database("keeper");
        Arc::new(Mutex::new(client))
    }
}