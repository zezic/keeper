extern crate dotenv;
use dotenv::dotenv;
use std::env;

use std::sync::Arc;
use tokio::sync::Mutex;
use clickhouse::Client;

pub type Db = Arc<Mutex<clickhouse::Client>>;

pub fn get_client() -> Db {
    dotenv().ok();

    let user = env::var("CLICKHOUSE_USER").unwrap();
    let password = env::var("CLICKHOUSE_PASSWORD").unwrap();
    let database = env::var("CLICKHOUSE_DATABASE").unwrap();
    
    let client = Client::default()
        .with_url("http://clickhouse:8123")
        .with_user(user)
        .with_password(password)
        .with_database(database);
    Arc::new(Mutex::new(client))
}