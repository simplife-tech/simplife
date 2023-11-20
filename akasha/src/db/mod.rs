pub mod instrumentation;

use sqlx::{mysql::{MySqlPoolOptions, MySqlConnectOptions}, MySql, Pool, ConnectOptions};

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Pool<MySql>
}

impl Db {
    pub fn new(pool: Pool<MySql>) -> Db {
        Self { pool }
    }
}
pub async fn db_connect(db_url: &str, max_connections: u32) -> Result<Pool<MySql>, sqlx::Error> {
    let options = db_url.parse();
    if options.is_err() {
        return Err(sqlx::Error::Protocol("invalid db url".to_string()));
    }
    let mut options: MySqlConnectOptions = options.unwrap();
    options.disable_statement_logging();
    MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options)
        .await
}
