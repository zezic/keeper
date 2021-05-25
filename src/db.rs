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