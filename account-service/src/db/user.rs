use sqlx::{FromRow, types::chrono::{DateTime, Utc}, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub family_id: Option<i64>,
    pub mobile: String,
    pub password: String,
    #[serde(serialize_with = "akasha::db::serialize_datetime")]
    pub ctime: Option<DateTime<Utc>>,
    #[serde(serialize_with = "akasha::db::serialize_datetime")]
    pub mtime: Option<DateTime<Utc>>
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
}