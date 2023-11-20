
use akasha::{db::Db, Context};
use chrono::{DateTime, Local};
use sqlx::{FromRow, Error};
use serde::{Deserialize, Serialize};


#[derive(Clone)]
pub struct ScheduleDao {
    db: Db,
}

const GET_USER_LEDGER_LIST_SQL: &str = "select id, uid, family_id, title, content, start_time, end_time, schedule_type, schedule_extra, state, ctime, mtime from schedule where (uid=? or (family_id>0 and family_id=?)) and start_time<? and end_time>? and state=0";

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Schedule {
    pub id: i64,
    pub uid: i64,
    pub family_id: i64,
    pub title: String,
    pub content: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub schedule_type: i64, // 0:普通;1:每周X;2:每月X号;3:每月最后一个工作日;4:每月最后一个休息日;5:每月第一个工作日;6:每月第一个休息日;7:每月最后一个周X;
    pub schedule_extra: String, 
    pub state: i64, // 0:正常;1:已删除
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
}

impl ScheduleDao {
    pub fn new(db: Db) -> ScheduleDao {
        ScheduleDao {
            db,
        }
    }
    
    pub async fn get_schedule_list(&self, ctx: &Context, uid: &i64, family_id: &i64, start_time: &DateTime<Local>, end_time: &DateTime<Local>) -> Result<Vec<Schedule>, Error> {
        match fetch_all!(ctx, &self.db.pool, Schedule, GET_USER_LEDGER_LIST_SQL, uid, family_id, end_time, start_time).await {
            Ok(schedules) => Ok(schedules),
            Err(err) => {
                log::error!("db error! {}", err);
                return Err(err)
            }
        }
    }

}