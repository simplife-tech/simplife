
use chrono::Local;
use sqlx::{FromRow, types::chrono::DateTime, Error};
use serde::{Deserialize, Serialize};

use super::Db;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Agenda {
    pub id: i64,
    pub uid: i64,
    pub family_id: i64,
    pub title: String,
    pub content: String,
    pub state: String,
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct AgendaLog {
    pub id: i64,
    pub uid: i64,
    pub agenda_id: i64,
    pub family_id: i64,
    pub title: String,
    pub content: String,
    pub state: String,
    pub ctime: DateTime<Local>,
}

const ADD_AGENDA_SQL: &str = "insert into agenda (uid, family_id, title, content, state, ctime, mtime) values (?, ?, ?, ?, ?, ?, ?)";
const DELETE_AGENDA_SQL: &str = "update agenda set state = 'deleted' where id = ?";
const GET_AGENDA_SQL: &str = "select id, uid, family_id, title, content, state, ctime, mtime from agenda where id = ?";
const UPDATE_AGENDA_SQL: &str = "update agenda set title=?, content=? where id = ?";
const GET_AGENDA_LIST_SQL: &str = "select id, uid, agenda_id, family_id, title, content, state, ctime, mtime from agenda where family_id=? and mtime between ? and ? and state='active' order by ctime desc limit ?, ?";

const ADD_AGENDA_LOG_SQL: &str = "insert into agenda_log (uid, agenda_id, family_id, title, content, state, ctime) values (?, ?, ?, ?, ?, ?, ?)";

impl Db {
    pub async fn add_agenda(&self, uid: &i64, family_id: &i64, title: &str, content: &str) -> Result<u64, Error> {
        let now = Local::now();
        match sqlx::query(ADD_AGENDA_SQL)
        .bind(uid)
        .bind(family_id)
        .bind(title)
        .bind(content)
        .bind("active")
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await {
            Ok(r) => Ok(r.last_insert_id()),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn delete_agenda(&self, id: &i64, uid: &i64) -> Result<u64, Error> {
        let mut tx = self.pool.begin().await?;
        let agenda = match sqlx::query_as::<_, Agenda>(GET_AGENDA_SQL)
        .bind(id)
        .fetch_one(&mut tx)
        .await {
            Ok(agenda) => agenda,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            },
        };
        match sqlx::query(DELETE_AGENDA_SQL)
        .bind(id)
        .execute(&mut tx)
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            }
        };
        let now = Local::now();
        let r = match sqlx::query(ADD_AGENDA_LOG_SQL)
        .bind(uid)
        .bind(id)
        .bind(agenda.family_id)
        .bind(agenda.title)
        .bind(agenda.content)
        .bind("deleted")
        .bind(now)
        .execute(&mut tx)
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            },
        };
        tx.commit().await?;
        Ok(r.last_insert_id())
    }

    pub async fn update_agenda(&self, id: &i64, uid: &i64, title: &str, content: &str) -> Result<u64, Error> {
        let mut tx = self.pool.begin().await?;
        let agenda = match sqlx::query_as::<_, Agenda>(GET_AGENDA_SQL)
        .bind(id)
        .fetch_one(&mut tx)
        .await {
            Ok(agenda) => agenda,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            },
        };
        match sqlx::query(UPDATE_AGENDA_SQL)
        .bind(title)
        .bind(content)
        .bind(id)
        .execute(&mut tx)
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            }
        };
        let now = Local::now();
        let r = match sqlx::query(ADD_AGENDA_LOG_SQL)
        .bind(uid)
        .bind(id)
        .bind(agenda.family_id)
        .bind(agenda.title)
        .bind(agenda.content)
        .bind(agenda.state)
        .bind(now)
        .execute(&mut tx)
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!("db error! {}", err);
                tx.rollback().await?;
                return Err(err)
            },
        };
        tx.commit().await?;
        Ok(r.last_insert_id())
    }

    pub async fn get_agenda(&self, id: &i64) -> Result<Option<Agenda>, Error> {
        match sqlx::query_as::<_, Agenda>(GET_AGENDA_SQL)
        .bind(id)
        .fetch_optional(&self.pool)
        .await {
            Ok(agenda) => Ok(agenda),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_agenda_list(&self, family_id: &i64, date_start: &DateTime<Local>, date_end: &DateTime<Local>, pn: &i64, ps: &i64) -> Result<Vec<Agenda>, Error> {
        match sqlx::query_as::<_, Agenda>(GET_AGENDA_LIST_SQL)
        .bind(family_id)
        .bind(date_start)
        .bind(date_end)
        .bind((pn-1)*ps)
        .bind(ps)
        .fetch_all(&self.pool)
        .await {
            Ok(agendas) => Ok(agendas),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}