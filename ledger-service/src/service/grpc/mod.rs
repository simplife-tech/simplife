pub mod proto;
pub mod client;
use redis::aio::ConnectionManager;
use sqlx::{MySql, Pool};
use tonic::{async_trait, Request, Response, Status, Code};
use crate::service::grpc::proto::v1::ledger_server::Ledger;
use crate::service::grpc::proto::v1::HelloWorldReq;
use crate::service::grpc::proto::v1::HelloWorldReply;
use crate::{cache::Redis, db::Db};

pub struct LedgerService {
    redis: Redis,
    db: Db,
}

impl LedgerService {
    pub fn new(pool: Pool<MySql>, redis: ConnectionManager) -> LedgerService {
        Self { db: Db::new(pool), redis: Redis::new(redis) }
    }
}

#[async_trait]
impl Ledger for LedgerService {
    async fn hello_world(&self, request: Request<HelloWorldReq>) -> Result<Response<HelloWorldReply>, Status> {
        Ok(Response::new(HelloWorldReply {n: 1}))
    }
}

