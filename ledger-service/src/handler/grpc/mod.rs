pub mod proto;
use akasha::db::Db;
use akasha::redis::Redis;
use tonic::{async_trait, Request, Response, Status};
use crate::handler::grpc::proto::v1::ledger_server::Ledger;
use crate::handler::grpc::proto::v1::HelloWorldReq;
use crate::handler::grpc::proto::v1::HelloWorldReply;

pub struct LedgerService {
    redis: Redis,
    db: Db,
}

impl LedgerService {
    pub fn new(db: Db, redis: Redis) -> LedgerService {
        Self { db, redis }
    }
}

#[async_trait]
impl Ledger for LedgerService {
    async fn hello_world(&self, request: Request<HelloWorldReq>) -> Result<Response<HelloWorldReply>, Status> {
        Ok(Response::new(HelloWorldReply {n: 1}))
    }
}

