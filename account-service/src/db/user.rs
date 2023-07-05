use sqlx::{FromRow, types::chrono::{DateTime, Local}, Error, Row};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub family_id: Option<i64>,
    pub mobile: String,
    pub password: String,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>
}

impl Db {
    pub async fn find_user_by_mobile(&self, mobile: &str) -> Result<User, Error> {
        match sqlx::query_as::<_, User>("select * from user where mobile=?")
        .bind(mobile)
        .fetch_one(&self.pool)
        .await {
            Ok(user) => Ok(user),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_id_by_uid(&self, uid: &i64) -> Result<Option<i64>, Error> {
        match sqlx::query("select family_id from user where id=?")
        .bind(uid)
        .fetch_one(&self.pool)
        .await {
            Ok(row) => Ok(row.get("family_id")),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }
}