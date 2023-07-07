
use chrono::Local;
use sqlx::{FromRow, types::chrono::DateTime, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Agenda {
    pub id: i64,
    pub uid: i64,
    pub family_id: Option<i64>,
    pub date: DateTime<Local>,
    pub ammount: i64,
    pub comment: String,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
    pub clazz_1: String,
    pub clazz_2: String,
}

impl Db {
    pub async fn add_ledger_with_uid(&self, uid: &i64, date: &DateTime<Local>, ammount: &i64, comment: &str, clazz_1: &str, clazz_2: &str) -> Result<u64, Error> {
        let now = Local::now();
        match sqlx::query("insert into ledger (uid, date, ammount, comment, ctime, mtime, clazz_1, clazz_2, state) values (?, ?, ?, ?, ?, ?, ?, ?, ?) ")
        .bind(uid)
        .bind(date)
        .bind(ammount)
        .bind(comment)
        .bind(now)
        .bind(now)
        .bind(clazz_1)
        .bind(clazz_2) 
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

    pub async fn delete_ledger(&self, id: &i64) -> Result<u64, Error> {
        match sqlx::query("update ledger set state = 'deleted' where id = ?")
        .bind(id)
        .execute(&self.pool)
        .await {
            Ok(r) => Ok(r.rows_affected()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_ledger(&self, id: &i64) -> Result<Option<Ledger>, Error> {
        match sqlx::query_as::<_, Ledger>("select * from ledger where id=?")
        .bind(id)
        .fetch_optional(&self.pool)
        .await {
            Ok(ledger) => Ok(ledger),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_ledger_list(&self, family_id: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match sqlx::query_as::<_, Ledger>("select * from ledger where family_id=? and date between ? and ? and state='active' order by date desc limit ?, ?")
        .bind(family_id)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        .await {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_user_ledger_list(&self, uid: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match sqlx::query_as::<_, Ledger>("select * from ledger where uid=? and date between ? and ? and state='active' order by date desc limit ?, ?")
        .bind(uid)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        .await {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}