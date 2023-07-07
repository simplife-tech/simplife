pub mod proto;
pub mod client;
use redis::aio::ConnectionManager;
use sqlx::{MySql, Pool};
use tonic::{async_trait, Request, Response, Status};
use crate::service::grpc::proto::v1::HelloWorldReq;
use crate::service::grpc::proto::v1::HelloWorldReply;
use crate::{cache::Redis, db::Db};

use self::proto::v1::agenda_server::Agenda;

pub struct AgendaService {
    redis: Redis,
    db: Db,
}

impl AgendaService {
    pub fn new(pool: Pool<MySql>, redis: ConnectionManager) -> AgendaService {
        Self { db: Db::new(pool), redis: Redis::new(redis) }
    }
}

#[async_trait]
impl Agenda for AgendaService {
    async fn hello_world(&self, request: Request<HelloWorldReq>) -> Result<Response<HelloWorldReply>, Status> {
        Ok(Response::new(HelloWorldReply {n: 1}))
    }
}

