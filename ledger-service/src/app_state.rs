
use crate::{cache::Redis, db::Db, service::grpc::client::GrpcClient};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
	pub redis: Redis,
    pub grpc_client: GrpcClient
}
