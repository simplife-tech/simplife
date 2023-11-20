use crate::{cache::Redis, db::Db};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
	pub redis: Redis
}
