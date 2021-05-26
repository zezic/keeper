extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::time::Duration;

use std::sync::Arc;
use tokio::sync::Mutex;
use clickhouse::Client;

use crate::types::{LogEntry};

pub type Db = Arc<Mutex<clickhouse::Client>>;
pub type Inserter = Arc<Mutex<clickhouse::inserter::Inserter<LogEntry>>>;

pub fn make_client() -> clickhouse::Client {
    dotenv().ok();

    let user = env::var("CLICKHOUSE_USER").unwrap();
    let password = env::var("CLICKHOUSE_PASSWORD").unwrap();
    let database = env::var("CLICKHOUSE_DATABASE").unwrap();
    
    let client = Client::default()
        .with_url("http://clickhouse:8123")
        .with_user(user)
        .with_password(password)
        .with_database(database);
    client
}

pub fn get_client() -> Db {
    Arc::new(Mutex::new(make_client()))
}

pub fn get_inserter() -> Inserter {
    let client = make_client();
    let inserter = client.inserter::<LogEntry>("entries").unwrap().with_max_duration(Duration::from_secs(5));
    Arc::new(Mutex::new(inserter))
}