pub mod proto;
use akasha::db::Db;
use akasha::redis::Redis;
use tonic::{async_trait, Request, Response, Status};
use crate::handler::grpc::proto::v1::agenda_server::Agenda;
use crate::handler::grpc::proto::v1::HelloWorldReq;
use crate::handler::grpc::proto::v1::HelloWorldReply;

pub struct AgendaService {
    redis: Redis,
    db: Db,
}

impl AgendaService {
    pub fn new(db: Db, redis: Redis) -> AgendaService {
        Self { db, redis }
    }
}

#[async_trait]
impl Agenda for AgendaService {
    async fn hello_world(&self, request: Request<HelloWorldReq>) -> Result<Response<HelloWorldReply>, Status> {
        Ok(Response::new(HelloWorldReply {n: 1}))
    }
}

