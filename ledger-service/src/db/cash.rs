use chrono::Local;
use sqlx::{FromRow, types::chrono::DateTime, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Cash {
    pub id: i64,
    pub family_id: i64,
    pub date: DateTime<Local>,
    pub ammount: i64,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
}

const ADD_CASH_RECORD_SQL: &str = "insert into cash (family_id, date, ammount, ctime, mtime, state) values (?, ?, ?, ?, ?, ?)";
const GET_CASH_RECORD_LIST_SQL: &str = "select id, family_id, date, ammount, ctime, mtime from cash where family_id=? and date between ? and ? and state='active' order by date desc limit ?, ?";
const DELETE_CASH_RECORD_SQL: &str = "update cash set state = 'deleted' where id = ?";
const GET_CASH_RECORD_SQL: &str = "select id, family_id, date, ammount, ctime, mtime from cash where id=?";

impl Db {
    pub async fn add_cash_record(&self, family_id: &i64, date: &DateTime<Local>, ammount: &i64) -> Result<u64, Error> {
        let now = Local::now();
        match sqlx::query(ADD_CASH_RECORD_SQL)
        .bind(family_id)
        .bind(date)
        .bind(ammount)
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

    pub async fn get_cash_record_list(&self, family_id: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Cash>, Error> {
        match sqlx::query_as::<_, Cash>(GET_CASH_RECORD_LIST_SQL)
        .bind(family_id)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        .await {
            Ok(cash_records) => Ok(cash_records),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn delete_cash_record(&self, id: &i64) -> Result<u64, Error> {
        match sqlx::query(DELETE_CASH_RECORD_SQL)
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

    pub async fn get_cash_record(&self, id: &i64) -> Result<Option<Cash>, Error> {
        match sqlx::query_as::<_, Cash>(GET_CASH_RECORD_SQL)
        .bind(id)
        .fetch_optional(&self.pool)
        .await {
            Ok(cash) => Ok(cash),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}