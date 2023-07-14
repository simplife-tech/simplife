use sqlx::{MySql, Pool};
pub mod ledger;
pub mod cash;

#[derive(Debug, Clone)]
pub struct Db {
    pool: Pool<MySql>
}

impl Db {
    pub fn new(pool: Pool<MySql>) -> Db {
        Self { pool }
    }
}