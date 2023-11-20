use akasha::{error::Error, dto::response::Response, redis::Redis, db::Db, Context};
use axum::{extract::Query, Extension, async_trait};
use grpc_client::account_service::GrpcAccountClient;
use crate::{service::ScheduleService, db::schedule::Schedule};

use crate::dto::schedule::GetScheduleReq;

#[derive(Clone)]
pub struct ScheduleHandler {
    pub schedule_service: ScheduleService,
    pub account_client: GrpcAccountClient
}

#[async_trait]
pub trait ScheduleHandlerTrait: Send + Clone + Sync + 'static {
    async fn get_schedule_list(&self, ctx: &Context, req: GetScheduleReq) -> Result<Vec<Schedule>, Error>;
}

impl ScheduleHandler {
    pub fn new(db: Db, redis: Redis, account_client: GrpcAccountClient) -> ScheduleHandler {
        let schedule_service = ScheduleService::new(db, redis, account_client.clone());
        Self { schedule_service, account_client }
    }
}

#[async_trait]
impl ScheduleHandlerTrait for ScheduleHandler {
    async fn get_schedule_list(&self, ctx: &Context, req: GetScheduleReq) -> Result<Vec<Schedule>, Error> {
        let uid = ctx.keys_i64.get("uid").unwrap();
        let family_id = match self.account_client.get_family_id(&ctx, &uid).await {
            Ok(family_id) => family_id,
            Err(_) => return Err(Error::NetWorkError),
        };
        match self.schedule_service.get_schedule_list(ctx, uid, &family_id, &req.start_time, &req.end_time).await {
            Ok(schedules) => {
                Ok(schedules)
            }
            Err(err) => {
                Err(Error::ServerError(err.to_string()))
            }
        }
    }
}

pub async fn get_schedule_list<T: ScheduleHandlerTrait>(
    Extension(handler): Extension<T>,
    Extension(ctx): Extension<Context>,
    Query(req): Query<GetScheduleReq>
) -> Result<Response<Vec<Schedule>>, Error> {
    match handler.get_schedule_list(&ctx, req).await {
        Ok(schedules) => Ok(Response::data(schedules)),
        Err(err) => Err(err)
    }
}

// pub async fn delete_ledger(
//     Extension(ctx): Extension<akasha::Context>,
//     State(state): State<AppState>,
//     Json(arg): Json<DeleteLedgerReq>
// ) -> Result<Response<String>, Error> {
//     let ledger_service = LedgerService::new(&ctx, &state.db, &state.redis, state.grpc_client.clone());
//     let uid = match state.grpc_client.get_uid(&ctx, &arg.access_key).await {
//         Ok(uid) => uid,
//         Err(_) => return Err(Error::NotLogin),
//     };
//     let family_id = match state.grpc_client.get_family_id(&ctx, &uid).await {
//         Ok(family_id) => family_id,
//         Err(_) => return Err(Error::NetWorkError),
//     };
//     match ledger_service.delete_ledger(uid, family_id, family_id).await {
//         Ok(_) => {
//             Ok(Response::success())
//         }
//         Err(err) => {
//             Err(err)
//         }
//     }
// }

// pub async fn ledger_list(
//     Extension(ctx): Extension<akasha::Context>,
//     State(state): State<AppState>,
//     Query(arg): Query<GetLedgerReq>
// ) -> Result<Response<Vec<LedgerDto>>, Error> {
//     if arg.pn<=0 || arg.ps <=0 {
//         return Err(Error::GetParamFailed)
//     }
//     let uid = match state.grpc_client.get_uid(&ctx, &arg.access_key).await {
//         Ok(uid) => uid,
//         Err(_) => return Err(Error::NotLogin),
//     };
//     let ledger_service = LedgerService::new(&ctx, &state.db, &state.redis, state.grpc_client.clone());
//     if arg.kind == "family" {
//         let family_id = match state.grpc_client.get_family_id(&ctx, &uid).await {
//             Ok(family_id) => family_id,
//             Err(_) => return Err(Error::NetWorkError),
//         };
//         if family_id > 0 {
//             match ledger_service.get_family_ledger(family_id, arg.date_start, arg.date_end, arg.pn, arg.ps).await {
//                 Ok(ledgers) => {
//                     let ledgers_dto: Vec<LedgerDto> = ledgers.iter().map(|ledger| LedgerDto {
//                         id: ledger.id,
//                         amount: ledger.amount,
//                         comment: ledger.comment.clone(),
//                         date: ledger.date.timestamp(),
//                         clazz_1: ledger.clazz_1.clone(),
//                         clazz_2: ledger.clazz_2.clone()
//                     }).collect();
//                     return Ok(Response::data(ledgers_dto))
//                 },
//                 Err(err) => return Err(err)
//             }
//         } else {
//             return Err(Error::NoFamily)
//         }
//     } else {
//         match ledger_service.get_user_ledger(uid, arg.date_start, arg.date_end, arg.pn, arg.ps).await {
//             Ok(ledgers) => {
//                 let ledgers_dto: Vec<LedgerDto> = ledgers.iter().map(|ledger| LedgerDto {
//                     id: ledger.id,
//                     amount: ledger.amount,
//                     comment: ledger.comment.clone(),
//                     date: ledger.date.timestamp(),
//                     clazz_1: ledger.clazz_1.clone(),
//                     clazz_2: ledger.clazz_2.clone()
//                 }).collect();
//                 return Ok(Response::data(ledgers_dto))
//             },
//             Err(err) => return Err(err)
//         }
//     }
// }