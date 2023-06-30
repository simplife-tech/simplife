use sqlx::{MySql, Pool};


#[derive(Debug, Clone)]
pub struct Db {
    pool: Pool<MySql>
}

impl Db {
    pub fn new(pool: Pool<MySql>) -> Db {
        Self { pool }
    }
}