#![deny(warnings)]

use warp::{Filter};
use tokio::sync::oneshot;
use tokio::signal::unix::{signal, SignalKind};
use futures::{pin_mut};

mod types;
mod handlers;
mod db;

#[tokio::main]
async fn main() {
    println!("Starting up...");
    pretty_env_logger::init();

    let db = db::get_client();
    let inserter = db::get_inserter();

    let api = filters::logger(db, inserter.clone());
    let routes = api.with(warp::log("Log Service"));
    let (tx, rx) = oneshot::channel();
    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 3030), async {
            rx.await.ok();
        });
    tokio::task::spawn(server);

    let mut term_stream = signal(SignalKind::terminate()).unwrap();
    let mut int_stream = signal(SignalKind::interrupt()).unwrap();

    let term_fut = term_stream.recv();
    let int_fut = int_stream.recv();

    pin_mut!(term_fut, int_fut);

    tokio::select! {
        Some(_) = term_fut => {}
        Some(_) = int_fut => {}
    }

    println!("Shutting down...");

    let _ = tx.send(());

    let mut inserter = inserter.lock().await;
    match inserter.commit().await {
        Ok(_) => { println!("Successufully commited insertion.") },
        Err(err) => { println!("Error when commiting insertion: {}", err) }
    };
}

mod filters {
    use super::db::{Db, Inserter};
    use super::handlers;
    use warp::Filter;

    pub fn logger(db: Db, inserter: Inserter) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        log_entry_post(inserter.clone())
            .or(log_entries_get(db.clone(), inserter.clone()))
    }

    pub fn log_entry_post(inserter: Inserter) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("log")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_inserter(inserter))
        .and_then(handlers::create_log_entry)
    }

    pub fn log_entries_get(db: Db, inserter: Inserter) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("log")
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db))
        .and(with_inserter(inserter))
        .and_then(handlers::list_log_entries)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn with_inserter(inserter: Inserter) -> impl Filter<Extract = (Inserter,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || inserter.clone())
    }
}