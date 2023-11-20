use akasha::{opentelemetry::trace::{Tracer, Span, TracerProvider}, instrumented_redis_cmd, redis::Redis};

#[derive(Clone)]
pub struct ScheduleCache {
    redis: Redis
}

// pub fn _user_ledger_key(uid: i64) -> String {
//     format!("LEDGER:USER:{}", uid.to_string())
// }

impl ScheduleCache {
    pub fn new(redis: Redis) -> ScheduleCache {
        ScheduleCache {
            redis
        }
    }

    // pub async fn set_user_ledger(&self, uid: i64, ledgers: &Vec<Ledger>, date_start: i64, date_end: i64, pn: i64, ps: i64) -> Result<(), RedisError> {
    //     let mut manager = self.manager.clone();
    //     let key = _user_ledger_key(uid);
    //     let field = _ledger_hset_field(date_start, date_end, pn, ps);
    //     match hset!(self.ctx, manager, &key, &field, serde_json::to_string(ledgers).unwrap()).await {
    //         Ok(_) => {
    //             match expire!(self.ctx, manager, &key, 60*60*1).await {
    //                 Ok(_) => Ok(()),
    //                 Err(err) => {
    //                     log::error!("redis error! {}", err);
    //                     return Err(err)
    //                 }
    //             }
    //         },
    //         Err(err) => {
    //             log::error!("redis error! {}", err);
    //             return Err(err)
    //         }
    //     }
    // }

}