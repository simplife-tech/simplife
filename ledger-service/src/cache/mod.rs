
#[derive(Debug, Clone)]
pub struct Redis {
    client: redis::Client
}

impl Redis {
    pub fn new(client: redis::Client) -> Redis {
        Self { client }
    }
}