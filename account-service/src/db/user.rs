use akasha::crypto::sha3_512;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
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
    pub async fn find_by_mobile_and_password(&self, mobile: &str, password: &str) -> Result<User, String> {
        let password_hash = sha3_512(password.to_string());
        let user = sqlx::query_as::<_, User>("select * from user where mobile=?")
            .bind(mobile)
            .fetch_one(&self.pool)
            .await;
        if user.is_err() {
            return Err("用户名或密码错误".to_string())
        }
        let user = user.unwrap();
        if user.password == password_hash {
            return Ok(user)
        } else {
            return Err("用户名或密码错误".to_string())
        }
    }
}