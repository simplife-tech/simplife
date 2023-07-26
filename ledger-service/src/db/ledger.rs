
use akasha::instrumented_mysql_cmd;
use akasha::opentelemetry::trace::{Tracer, Span, TracerProvider};
use chrono::{Local, NaiveDateTime};
use sqlx::{FromRow, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Ledger {
    pub id: i64,
    pub uid: i64,
    pub family_id: Option<i64>,
    pub date: NaiveDateTime,
    pub amount: i64,
    pub comment: String,
    pub ctime: NaiveDateTime,
    pub mtime: NaiveDateTime,
    pub clazz_1: String,
    pub clazz_2: String,
}

const ADD_USER_LEDGER_SQL: &str = "insert into ledger (uid, date, amount, comment, ctime, mtime, clazz_1, clazz_2, state) values (?, ?, ?, ?, ?, ?, ?, ?, ?)";
const ADD_FAMILY_LEDGER_SQL: &str = "insert into ledger (uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2, state) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
const DELETE_LEDGER_SQL: &str = "update ledger set state = 'deleted' where id = ?";
const GET_LEDGER_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where id=?";
const GET_FAMILY_LEDGER_LIST_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where family_id=? and date between ? and ? and state='active' order by date desc limit ?, ?";
const GET_USER_LEDGER_LIST_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where uid=? and date between ? and ? and state='active' order by date desc limit ?, ?";

impl Db {
    pub async fn add_ledger_with_uid(&self, oc: &akasha::opentelemetry::Context, uid: &i64, date: &NaiveDateTime, amount: &i64, comment: &str, clazz_1: &str, clazz_2: &str) -> Result<u64, Error> {
        let now = Local::now().naive_local();
        match instrumented_mysql_cmd!(oc, ADD_USER_LEDGER_SQL, sqlx::query(ADD_USER_LEDGER_SQL)
        .bind(uid)
        .bind(date)
        .bind(amount)
        .bind(comment)
        .bind(now)
        .bind(now)
        .bind(clazz_1)
        .bind(clazz_2) 
        .bind("active")
        .execute(&self.pool)
        ) {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn add_ledger_with_uid_and_family_id(&self, oc: &akasha::opentelemetry::Context, uid: &i64, family_id: &i64, date: &NaiveDateTime, amount: &i64, comment: &str, clazz_1: &str, clazz_2: &str) -> Result<u64, Error> {
        let now = Local::now().naive_local();
        match instrumented_mysql_cmd!(oc, ADD_FAMILY_LEDGER_SQL, sqlx::query(ADD_FAMILY_LEDGER_SQL)
        .bind(uid)
        .bind(family_id)
        .bind(date)
        .bind(amount)
        .bind(comment)
        .bind(now)
        .bind(now)
        .bind(clazz_1)
        .bind(clazz_2) 
        .bind("active")
        .execute(&self.pool)
        ) {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn delete_ledger(&self, oc: &akasha::opentelemetry::Context, id: &i64) -> Result<u64, Error> {
        match instrumented_mysql_cmd!(oc, DELETE_LEDGER_SQL, sqlx::query(DELETE_LEDGER_SQL)
        .bind(id)
        .execute(&self.pool)
        ) {
            Ok(r) => Ok(r.rows_affected()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_ledger(&self, oc: &akasha::opentelemetry::Context, id: &i64) -> Result<Option<Ledger>, Error> {
        match instrumented_mysql_cmd!(oc, GET_LEDGER_SQL, sqlx::query_as::<_, Ledger>(GET_LEDGER_SQL)
        .bind(id)
        .fetch_optional(&self.pool)
        ) {
            Ok(ledger) => Ok(ledger),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_ledger_list(&self, oc: &akasha::opentelemetry::Context, family_id: &i64, date_start: &NaiveDateTime, date_end: &NaiveDateTime, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match instrumented_mysql_cmd!(oc, GET_FAMILY_LEDGER_LIST_SQL, sqlx::query_as::<_, Ledger>(GET_FAMILY_LEDGER_LIST_SQL)
        .bind(family_id)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        ) {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_user_ledger_list(&self, oc: &akasha::opentelemetry::Context, uid: &i64, date_start: &NaiveDateTime, date_end: &NaiveDateTime, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match instrumented_mysql_cmd!(oc, GET_USER_LEDGER_LIST_SQL, sqlx::query_as::<_, Ledger>(GET_USER_LEDGER_LIST_SQL)
        .bind(uid)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        ) {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}