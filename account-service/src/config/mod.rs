
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

lazy_static! {
    pub static ref GLOBAL_CONFIG: RwLock<Config> = RwLock::new(Config {
        server: Server {
            listen_ip: "127.0.0.1".to_string(),
            listen_port: 27001
        },
        db: Db {
            url: "".to_string(),
            max_connections: 5
        },
        service: Service {
            session_expired_time: 300
        }
    });
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub server: Server,
    pub db: Db,
    pub service: Service
}

#[derive(Deserialize, Serialize)]
pub struct Server {
    pub listen_ip: String,
    pub listen_port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Db {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Service {
    pub session_expired_time: i32,
}