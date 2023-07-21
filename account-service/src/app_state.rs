use crate::{db::Db, cache::Redis};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
	pub redis: Redis,
}
