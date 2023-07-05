
use chrono::Local;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Ledger {
    pub id: i64,
    pub uid: i64,
    pub family_id: Option<i64>,
    pub date: DateTime<Local>,
    pub ammount: i64,
    pub comment: String,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>
}

impl Db {
    pub async fn add_ledger_with_uid(&self, uid: &i64, date: &DateTime<Local>, ammount: &i64, comment: &str) -> Result<u64, Error> {
        let now = Local::now();
        match sqlx::query("insert into ledger (uid, date, ammount, comment, ctime, mtime, state) values (?, ?, ?, ?, ?, ?, ?) ")
        .bind(uid)
        .bind(date)
        .bind(ammount)
        .bind(comment)
        .bind(now)
        .bind(now) 
        .bind("active")
        .execute(&self.pool)
        .await {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn add_ledger_with_uid_and_family_id(&self, uid: &i64, family_id: &i64, date: &DateTime<Local>, ammount: &i64, comment: &str) -> Result<u64, Error> {
        let now = Local::now();
        match sqlx::query("insert into ledger (uid, family_id, date, ammount, comment, ctime, mtime, state) values (?, ?, ?, ?, ?, ?, ?, ?) ")
        .bind(uid)
        .bind(family_id)
        .bind(date)
        .bind(ammount)
        .bind(comment)
        .bind(now)
        .bind(now) 
        .bind("active")
        .execute(&self.pool)
        .await {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}