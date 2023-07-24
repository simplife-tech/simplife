use sqlx::{FromRow, types::chrono::NaiveDateTime, Error, Row};
use serde::{Deserialize, Serialize};
use akasha::opentelemetry::trace::{Tracer, Span, TracerProvider};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub family_id: Option<i64>,
    pub mobile: String,
    pub password: String,
    pub ctime: NaiveDateTime,
    pub mtime: NaiveDateTime
}

const SELECT_USER_BY_MOBILE_SQL: &str = "select * from user where mobile=?";
const SELECT_FAMILY_ID_BY_UID: &str = "select family_id from user where id=?";

impl Db {
    pub async fn find_user_by_mobile(&self, oc: &akasha::opentelemetry::Context, mobile: &str) -> Result<User, Error> {
        match instrumented_mysql_cmd!(oc, SELECT_USER_BY_MOBILE_SQL, sqlx::query_as::<_, User>(SELECT_USER_BY_MOBILE_SQL)
        .bind(mobile)
        .fetch_one(&self.pool)
        ) {
            Ok(user) => Ok(user),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_id_by_uid(&self, oc: &akasha::opentelemetry::Context, uid: &i64) -> Result<Option<i64>, Error> {
        match instrumented_mysql_cmd!(oc, SELECT_FAMILY_ID_BY_UID, sqlx::query(SELECT_FAMILY_ID_BY_UID)
        .bind(uid)
        .fetch_one(&self.pool)
        ) {
            Ok(row) => Ok(row.get("family_id")),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }
}