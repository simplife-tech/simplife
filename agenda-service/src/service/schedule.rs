use akasha::{db::Db, redis::Redis, Context, time::timestamp_to_datetime};
use grpc_client::account_service::GrpcAccountClient;

use crate::{db::schedule::{ScheduleDao, Schedule}, cache::schedule::ScheduleCache, dto::error::Error};


#[derive(Clone)]
pub struct ScheduleService {
    schedule_dao: ScheduleDao,
    schedule_cache: ScheduleCache,
    account_client: GrpcAccountClient
}

impl ScheduleService {
    pub fn new(db: Db, redis: Redis, account_client: GrpcAccountClient) -> ScheduleService {
        ScheduleService {
            schedule_dao: ScheduleDao::new(db),
            schedule_cache: ScheduleCache::new(redis),
            account_client
        }
    }

    pub async fn get_schedule_list(&self, ctx: &Context, uid: &i64, family_id: &i64, start_time: &i64, end_time: &i64) -> Result<Vec<Schedule>, Error> {
        match self.schedule_dao.get_schedule_list(ctx, uid, family_id, &timestamp_to_datetime(*start_time), &timestamp_to_datetime(*end_time)).await {
            Ok(schedules) => {
                // let _ = self.ledger_cache.set_family_ledger(family_id, &ledgers, date_start, date_end, pn, ps).await;
                return Ok(schedules)
            },
            Err(_) => {
                return Err(Error::NetWorkError)
            }
        }
    }

    // pub async fn get_user_ledger(&self, uid: i64, date_start: i64, date_end:i64, pn: i64, ps: i64 ) -> Result<Vec<Ledger>, Error> {
    //     let ledgers = match self.ledger_cache.get_user_ledger(uid, date_start, date_end, pn, ps).await {
    //         Ok(ledgers) => ledgers,
    //         Err(_) => None
    //     };
    //     if let Some(ledgers) = ledgers {
    //         return Ok(ledgers)
    //     }
    //     match self.ledger_dao.get_user_ledger_list(uid, &akasha::time::timestamp_to_datetime(date_start), &akasha::time::timestamp_to_datetime(date_end), pn, ps).await {
    //         Ok(ledgers) => {
    //             let _ = self.ledger_cache.set_user_ledger(uid, &ledgers, date_start, date_end, pn, ps).await;
    //             return Ok(ledgers)
    //         },
    //         Err(_) => {
    //             return Err(Error::NetWorkError)
    //         }
    //     }
    // }
}