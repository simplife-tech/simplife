use akasha::{db::Db, redis::Redis, Context, error::Error};
use grpc_client::account_service::GrpcAccountClient;

use crate::{dto::error, db::ledger::{LedgerDao, Ledger}, cache::ledger::LedgerCache, model::{LEDGER_CLAZZ_1, LEDGER_CLAZZ_2}};


#[derive(Clone)]
pub struct LedgerService {
    ledger_dao: LedgerDao,
    ledger_cache: LedgerCache,
    grpc_client: GrpcAccountClient
}

impl LedgerService {
    pub fn new(db: Db, redis: Redis, grpc_client: GrpcAccountClient) -> LedgerService {
        LedgerService {
            ledger_dao: LedgerDao::new(db),
            ledger_cache: LedgerCache::new(redis),
            grpc_client: grpc_client
        }
    }

    pub async fn add_ledger(&self, ctx: &Context, uid: &i64, family_id: &i64, timestamp: &i64, amount: &i64, comment: &str, clazz_1: &i64, clazz_2: &i64) -> Result<(), Error> {
        if LEDGER_CLAZZ_1.get(clazz_1).is_none() || LEDGER_CLAZZ_2.get(clazz_2).is_none() {
            return Err(Error::BadRequest)
        }
        if *family_id <= 0 {
            match self.ledger_dao.add_ledger_with_uid(ctx, uid, &akasha::time::timestamp_to_datetime(*timestamp), amount, comment, clazz_1, clazz_2).await {
                Ok(_) => {
                    let _ = self.ledger_cache.remove_user_ledger(ctx, uid).await;
                    Ok(())
                },
                Err(_) => {
                    Err(Error::NetWorkError)
                }
            }
        } else {
            match self.ledger_dao.add_ledger_with_uid_and_family_id(ctx, uid, family_id, &akasha::time::timestamp_to_datetime(*timestamp), amount, comment, clazz_1, clazz_2).await {
                Ok(_) => {
                    let _ = self.ledger_cache.remove_user_ledger(ctx, uid).await;
                    let _ = self.ledger_cache.remove_family_ledger(ctx, family_id).await;
                    Ok(())
                },
                Err(_) => {
                    Err(Error::NetWorkError)
                }
            }
        }
    }

    pub async fn delete_ledger(&self, ctx: &Context, id: &i64, uid: &i64, family_id: &i64) -> Result<(), Error> {
        let ledger = match self.ledger_dao.get_ledger(ctx, id).await {
            Ok(ledger) => {
                match ledger {
                    Some(ledger) => ledger,
                    None => return Err(error::NO_LEDGER)
                }
            },
            Err(_) => return Err(Error::NetWorkError)

        };
        if ledger.uid != *uid {
            return Err(error::DELETE_LEDGER_NOT_SAME_UID)
        }
        match self.ledger_dao.delete_ledger(ctx, &ledger.id).await {
            Ok(_) => {
                let _ = self.ledger_cache.remove_user_ledger(ctx, uid).await;
                if *family_id > 0 {
                    let _ = self.ledger_cache.remove_family_ledger(ctx, family_id).await;
                }
                return Ok(())
            },
            Err(_) => return Err(Error::NetWorkError)
        }
    }

    pub async fn get_family_ledger(&self, ctx: &Context, family_id: &i64, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<Vec<Ledger>, Error> {
        let ledgers = match self.ledger_cache.get_family_ledger(ctx, family_id, date_start, date_end, pn, ps).await {
            Ok(ledgers) => ledgers,
            Err(_) => None
        };
        if let Some(ledgers) = ledgers {
            return Ok(ledgers)
        }
        match self.ledger_dao.get_family_ledger_list(ctx, family_id, &akasha::time::timestamp_to_datetime(*date_start), &akasha::time::timestamp_to_datetime(*date_end), pn, ps).await {
            Ok(ledgers) => {
                let _ = self.ledger_cache.set_family_ledger(ctx, family_id, &ledgers, date_start, date_end, pn, ps).await;
                return Ok(ledgers)
            },
            Err(_) => {
                return Err(Error::NetWorkError)
            }
        }
    }

    pub async fn get_user_ledger(&self, ctx: &Context, uid: &i64, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64 ) -> Result<Vec<Ledger>, Error> {
        let ledgers = match self.ledger_cache.get_user_ledger(ctx, uid, date_start, date_end, pn, ps).await {
            Ok(ledgers) => ledgers,
            Err(_) => None
        };
        if let Some(ledgers) = ledgers {
            return Ok(ledgers)
        }
        match self.ledger_dao.get_user_ledger_list(ctx, uid, &akasha::time::timestamp_to_datetime(*date_start), &akasha::time::timestamp_to_datetime(*date_end), pn, ps).await {
            Ok(ledgers) => {
                let _ = self.ledger_cache.set_user_ledger(ctx, uid, &ledgers, date_start, date_end, pn, ps).await;
                return Ok(ledgers)
            },
            Err(_) => {
                return Err(Error::NetWorkError)
            }
        }
    }
}