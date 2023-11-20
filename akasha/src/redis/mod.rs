pub mod instrumentation;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct Redis {
    pub manager: ConnectionManager,
}

impl Redis {
    pub fn new(manager: ConnectionManager) -> Redis {
        Self { manager }
    }
}