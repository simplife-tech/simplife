
use akasha::{db::Db, Context};
use chrono::{Local, DateTime};
use sqlx::{FromRow, Error};
use serde::{Deserialize, Serialize};


#[derive(Clone)]
pub struct LedgerDao {
    db: Db
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Ledger {
    pub id: i64,
    pub uid: i64,
    pub family_id: Option<i64>,
    pub date: DateTime<Local>,
    pub amount: i64,
    pub comment: String,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
    pub clazz_1: i64,
    pub clazz_2: i64,
}

const ADD_USER_LEDGER_SQL: &str = "insert into ledger (uid, date, amount, comment, ctime, mtime, clazz_1, clazz_2, state) values (?, ?, ?, ?, ?, ?, ?, ?, ?)";
const ADD_FAMILY_LEDGER_SQL: &str = "insert into ledger (uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2, state) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
const DELETE_LEDGER_SQL: &str = "update ledger set state = 'deleted' where id = ?";
const GET_LEDGER_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where id=?";
const GET_FAMILY_LEDGER_LIST_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where family_id=? and date between ? and ? and state='active' order by date desc limit ?, ?";
const GET_USER_LEDGER_LIST_SQL: &str = "select id, uid, family_id, date, amount, comment, ctime, mtime, clazz_1, clazz_2 from ledger where uid=? and date between ? and ? and state='active' order by date desc limit ?, ?";

impl LedgerDao {
    pub fn new(db: Db) -> LedgerDao {
        LedgerDao {
            db,
        }
    }
    
    pub async fn add_ledger_with_uid(&self, ctx: &Context, uid: &i64, date: &DateTime<Local>, amount: &i64, comment: &str, clazz_1: &i64, clazz_2: &i64) -> Result<u64, Error> {
        let now = Local::now();
        match execute!(ctx, &self.db.pool, ADD_USER_LEDGER_SQL, uid, date, amount, comment, now, now, clazz_1, clazz_2, "active").await {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn add_ledger_with_uid_and_family_id(&self, ctx: &Context, uid: &i64, family_id: &i64, date: &DateTime<Local>, amount: &i64, comment: &str, clazz_1: &i64, clazz_2: &i64) -> Result<u64, Error> {
        let now = Local::now();
        match execute!(ctx, &self.db.pool, ADD_FAMILY_LEDGER_SQL,
        uid,
        family_id,
        date,
        amount,
        comment,
        now,
        now,
        clazz_1,
        clazz_2,
        "active").await {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn delete_ledger(&self, ctx: &Context, id: &i64) -> Result<u64, Error> {
        match execute!(ctx, &self.db.pool, DELETE_LEDGER_SQL, id).await {
            Ok(r) => Ok(r.rows_affected()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_ledger(&self, ctx: &Context, id: &i64) -> Result<Option<Ledger>, Error> {
        match fetch_optional!(ctx, &self.db.pool, Ledger, GET_LEDGER_SQL, id).await {
            Ok(ledger) => Ok(ledger),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_ledger_list(&self, ctx: &Context, family_id: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match fetch_all!(ctx, &self.db.pool, Ledger, GET_FAMILY_LEDGER_LIST_SQL, family_id, date_start, date_end, (pn-1)*ps, ps).await {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_user_ledger_list(&self, ctx: &Context, uid: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        match fetch_all!(ctx, &self.db.pool, Ledger, GET_USER_LEDGER_LIST_SQL, uid, date_start, date_end, (pn-1)*ps, ps).await {
            Ok(ledgers) => Ok(ledgers),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}