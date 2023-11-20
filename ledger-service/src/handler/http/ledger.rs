use akasha::{dto::response::Response, db::Db, redis::Redis, Context, error::Error};
use axum::{extract::Query, Json, Extension, async_trait};
use grpc_client::account_service::GrpcAccountClient;

use crate::{dto::ledger::{AddLedgerReq, LedgerListReq, DeleteLedgerReq, LedgerDto}, service::LedgerService};

#[derive(Clone)]
pub struct LedgerHandler {
    pub ledger_service: LedgerService,
    pub grpc_client: GrpcAccountClient
}

#[async_trait]
pub trait LedgerHandlerTrait: Send + Clone + Sync + 'static {
    async fn add_ledger(&self, ctx: &Context, req: AddLedgerReq) -> Result<String, Error>;
    async fn delete_ledger(&self, ctx: &Context, req: DeleteLedgerReq) -> Result<String, Error>;
    async fn ledger_list(&self, ctx: &Context, req: LedgerListReq) -> Result<Vec<LedgerDto>, Error>;
}

impl LedgerHandler {
    pub fn new(db: Db, redis: Redis, grpc_client: GrpcAccountClient) -> LedgerHandler {
        let ledger_service = LedgerService::new(db, redis, grpc_client.clone());
        Self { ledger_service, grpc_client }
    }
}

#[async_trait]
impl LedgerHandlerTrait for LedgerHandler {
    async fn add_ledger(&self, ctx: &Context, req: AddLedgerReq) -> Result<String, Error> {
        let uid = ctx.keys_i64.get("uid").unwrap();
        let family_id = match self.grpc_client.get_family_id(&ctx, uid).await {
            Ok(family_id) => family_id,
            Err(_) => return Err(Error::NetWorkError),
        };
        match self.ledger_service.add_ledger(ctx, &uid, &family_id, &req.date, &req.amount, &req.comment, &req.clazz_1, &req.clazz_2).await {
            Ok(_) => {
                Ok("success".to_string())
            }
            Err(err) => {
                Err(Error::ServerError(err.to_string()))
            }
        }
    }

    async fn delete_ledger(&self, ctx: &Context, req: DeleteLedgerReq) -> Result<String, Error> {
        let uid = ctx.keys_i64.get("uid").unwrap();
        let family_id = match self.grpc_client.get_family_id(&ctx, &uid).await {
            Ok(family_id) => family_id,
            Err(_) => return Err(Error::NetWorkError),
        };
        match self.ledger_service.delete_ledger(ctx, &req.id, uid, &family_id).await {
            Ok(_) => {
                Ok("success".to_string())
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    async fn ledger_list(&self, ctx: &Context, req: LedgerListReq) -> Result<Vec<LedgerDto>, Error> {
        if req.pn<=0 || req.ps <=0 {
            return Err(Error::BadRequest)
        }
        let uid = ctx.keys_i64.get("uid").unwrap();
        if req.kind == "family" {
            let family_id = match self.grpc_client.get_family_id(&ctx, &uid).await {
                Ok(family_id) => family_id,
                Err(_) => return Err(Error::NetWorkError),
            };
            if family_id > 0 {
                match self.ledger_service.get_family_ledger(ctx, &family_id, &req.date_start, &req.date_end, &req.pn, &req.ps).await {
                    Ok(ledgers) => {
                        let ledgers_dto: Vec<LedgerDto> = ledgers.iter().map(|ledger| LedgerDto {
                            id: ledger.id,
                            uid: ledger.uid,
                            family_id: ledger.family_id.unwrap_or(-1),
                            amount: ledger.amount,
                            comment: ledger.comment.clone(),
                            date: ledger.date.timestamp(),
                            clazz_1: ledger.clazz_1,
                            clazz_2: ledger.clazz_2
                        }).collect();
                        return Ok(ledgers_dto)
                    },
                    Err(err) => return Err(err)
                }
            } else {
                return Err(Error::UserHasNoFamily)
            }
        } else {
            match self.ledger_service.get_user_ledger(ctx, uid, &req.date_start, &req.date_end, &req.pn, &req.ps).await {
                Ok(ledgers) => {
                    let ledgers_dto: Vec<LedgerDto> = ledgers.iter().map(|ledger| LedgerDto {
                        id: ledger.id,
                        uid: ledger.uid,
                        family_id: ledger.family_id.unwrap_or(-1),
                        amount: ledger.amount,
                        comment: ledger.comment.clone(),
                        date: ledger.date.timestamp(),
                        clazz_1: ledger.clazz_1.clone(),
                        clazz_2: ledger.clazz_2.clone()
                    }).collect();
                    return Ok(ledgers_dto)
                },
                Err(err) => return Err(err)
            }
        }
    }


}

pub async fn add_ledger<T: LedgerHandlerTrait>(
    Extension(handler): Extension<T>,
    Extension(ctx): Extension<Context>,
    Json(req): Json<AddLedgerReq>
) -> Result<Response<String>, Error> {
    match handler.add_ledger(&ctx, req).await {
        Ok(login_reply) => Ok(Response::data(login_reply)),
        Err(err) => Err(err)
    }
}

pub async fn delete_ledger<T: LedgerHandlerTrait>(
    Extension(handler): Extension<T>,
    Extension(ctx): Extension<Context>,
    Json(req): Json<DeleteLedgerReq>
) -> Result<Response<String>, Error> {
    match handler.delete_ledger(&ctx, req).await {
        Ok(delete_reply) => Ok(Response::data(delete_reply)),
        Err(err) => Err(err)
    }
}

pub async fn ledger_list<T: LedgerHandlerTrait>(
    Extension(handler): Extension<T>,
    Extension(ctx): Extension<Context>,
    Query(req): Query<LedgerListReq>
) -> Result<Response<Vec<LedgerDto>>, Error> {
    match handler.ledger_list(&ctx, req).await {
        Ok(ledgers) => Ok(Response::data(ledgers)),
        Err(err) => Err(err)
    }
}