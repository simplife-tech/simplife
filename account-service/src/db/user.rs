use akasha::{db::Db, Context};
use sqlx::{FromRow, types::chrono::NaiveDateTime, Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct UserDao {
    db: Db,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub family_id: Option<i64>,
    pub mobile: String,
    pub password: String,
    pub ctime: NaiveDateTime,
    pub mtime: NaiveDateTime
}

const SELECT_USER_BY_MOBILE_SQL: &str = "select id, family_id, mobile, password, ctime, mtime from user where mobile=?";
const SELECT_FAMILY_ID_BY_UID_SQL: &str = "select family_id from user where id=?";
impl UserDao {
    pub fn new(db: Db) -> UserDao {
        UserDao {
            db,
        }
    }

    pub async fn find_user_by_mobile(&self, ctx: &Context, mobile: &str) -> Result<User, Error> {  
        match fetch_one!(ctx, &self.db.pool, User, SELECT_USER_BY_MOBILE_SQL, mobile).await {
            Ok(user) => Ok(user),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_id_by_uid(&self, ctx: &Context, uid: &i64) -> Result<Option<i64>, Error> {
        match fetch_one!(ctx, &self.db.pool, SELECT_FAMILY_ID_BY_UID_SQL, uid).await {
            Ok(row) => Ok(row.get("family_id")),
            Err(err) => {
                if let Error::RowNotFound = err {
                    return Ok(None)
                }
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }
}