use redis::{AsyncCommands, RedisError};
use akasha::{redis::Redis, Context};
use crate::db::ledger::Ledger;

#[derive(Clone)]
pub struct LedgerCache {
    redis: Redis
}

pub fn _user_ledger_key(uid: &i64) -> String {
    format!("LEDGER:USER:{}", uid.to_string())
}

pub fn _family_ledger_key(family_id: &i64) -> String {
    format!("LEDGER:FAMILY:{}", family_id.to_string())
}

pub fn _ledger_hset_field(date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> String {
    format!("DATE_{}_{}_PN_{}_PS_{}", date_start.to_string(), date_end.to_string(), pn.to_string(), ps.to_string())
}

impl LedgerCache {
    pub fn new(redis: Redis) -> LedgerCache {
        LedgerCache {
            redis
        }
    }

    pub async fn set_user_ledger(&self, ctx: &Context, uid: &i64, ledgers: &Vec<Ledger>, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<(), RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _user_ledger_key(&uid);
        let field = _ledger_hset_field(date_start, date_end, pn, ps);
        match hset!(ctx, manager, &key, &field, serde_json::to_string(ledgers).unwrap()).await {
            Ok(_) => {
                match expire!(ctx, manager, &key, 60*60*1).await {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                }
            },
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_user_ledger(&self, ctx: &Context, uid: &i64, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<Option<Vec<Ledger>>, RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _user_ledger_key(uid);
        let field = _ledger_hset_field(date_start, date_end, pn, ps);
        let exist: i8 = match hexists!(ctx, manager, &key, &field).await {
            Ok(exist) => exist,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
        if exist == 1 {
            let s: String = match hget!(ctx, manager, &key, &field).await {
                Ok(s) => s,
                Err(err) => {
                    log::error!("redis error! {}", err);
                    return Err(err)
                }
            };
            let ledgers: Vec<Ledger> = serde_json::from_str(&s).unwrap_or(vec![]);
            return Ok(Some(ledgers))
        } else {
            return Ok(None)
        }
    }

    pub async fn remove_user_ledger(&self, ctx: &Context, uid: &i64) -> Result<(), RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _user_ledger_key(uid);
        match del!(ctx, manager, &key).await {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("redis error! {}", err);
                Err(err)
            }
        }
    }

    pub async fn set_family_ledger(&self, ctx: &Context, family_id: &i64, ledgers: &Vec<Ledger>, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<(), RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _family_ledger_key(family_id);
        let field = _ledger_hset_field(date_start, date_end, pn, ps);
        match hset!(ctx, manager, &key, &field, serde_json::to_string(ledgers).unwrap()).await {
            Ok(_) => {
                match expire!(ctx, manager, &key, 60*60*1).await {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                }
            },
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_family_ledger(&self, ctx: &Context, family_id: &i64, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<Option<Vec<Ledger>>, RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _family_ledger_key(family_id);
        let field = _ledger_hset_field(date_start, date_end, pn, ps);
        let exist: i8 = match hexists!(ctx, manager, &key, &field).await {
            Ok(exist) => exist,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
        if exist == 1 {
            let s: String = match hget!(ctx, manager, &key, &field).await {
                Ok(s) => s,
                Err(err) => {
                    log::error!("redis error! {}", err);
                    return Err(err)
                }
            };
            let ledgers: Vec<Ledger> = serde_json::from_str(&s).unwrap_or(vec![]);
            return Ok(Some(ledgers))
        } else {
            return Ok(None)
        }
    }

    pub async fn remove_family_ledger(&self, ctx: &Context, family_id: &i64) -> Result<(), RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _family_ledger_key(family_id);
        match del!(ctx, manager, &key).await {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("redis error! {}", err);
                Err(err)
            }
        }
    }
}