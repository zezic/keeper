#![deny(warnings)]

use warp::{Filter};
#[macro_use]
extern crate num_derive;

mod types;
mod handlers;
mod db;

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