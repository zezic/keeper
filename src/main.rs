#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};

use warp::Filter;

#[derive(Clone, Deserialize, Serialize)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LogEntry {
    level: LogLevel,
    message: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = db::blank_db();

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
    use super::LogEntry;
    use std::convert::Infallible;

    pub async fn create_log_entry(entry: LogEntry, db: Db) -> Result<impl warp::Reply, Infallible> {
        let mut entries = db.lock().await;
        entries.push(entry.clone());
        Ok(warp::reply::json(&entry))
    }

    pub async fn list_log_entries(db: Db) -> Result<impl warp::Reply, Infallible> {
        let entries = db.lock().await;
        let derefed = &*entries;
        Ok(warp::reply::json(&derefed))
    }
}

mod db {
    use super::LogEntry;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<LogEntry>>>;

    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }
}