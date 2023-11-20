use akasha::{multiplex_service::MultiplexService, db::{db_connect, Db}, redis::Redis};
use config::GLOBAL_CONFIG;
mod handler;
use grpc_client::account_service::GrpcAccountClient;
use handler::{grpc::LedgerService, http::{new_ledger_router, ledger::LedgerHandler}};
use std::{net::{SocketAddr, IpAddr}, str::FromStr};
use tower::make::Shared;
use crate::handler::grpc::proto::v1::ledger_server::LedgerServer;
mod db;
use tonic::transport::Server;
use hyper;
use clap::Parser;
mod config;
mod model;
mod cache;
mod dto;
mod service;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate akasha;  

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db: String,

    #[arg(long, default_value_t = 5)]
    db_max_connections: u32,

    #[arg(short, long)]
    redis: String,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    listen_ip: String,

    #[arg(long, default_value_t = 27001)]
    listen_port: u16,

    #[arg(long, default_value_t = String::from("http://localhost:27001"))]
    account_service: String,

    #[arg(long, default_value_t = String::from("api.simplife.tech:6831"))]
    trace_endpoint: String,

    #[arg(long, default_value_t = String::from("test"))]
    service_version: String,
}

#[tokio::main]
async fn main() {
    akasha::log::init_config(log::LevelFilter::Info);
    let args = Args::parse();
    akasha::app::init_tracer(args.trace_endpoint, "ledger.service".to_string(), args.service_version);

    {
        let mut config = GLOBAL_CONFIG.write().await;
        config.db.url = args.db.clone();
        config.db.max_connections = args.db_max_connections.clone();
        config.server.listen_ip = args.listen_ip.clone();
        config.server.listen_port = args.listen_port.clone();
    }

    let pool = match db_connect(&GLOBAL_CONFIG.read().await.db.url, GLOBAL_CONFIG.read().await.db.max_connections).await {
        Ok(pool) => pool,
        Err(err) => {
            panic!("connect db error! {}", err)
        }
    };
    
    let redis = match redis::Client::open(args.redis) {
        Ok(client) => match redis::aio::ConnectionManager::new(client).await {
            Ok(manager) => manager,
            Err(err) => {
                panic!("connect redis error! {}", err)
            }
        },
        Err(err) => {
            panic!("connect redis error! {}", err)
        }
    };

    let db: Db = Db::new(pool);
    let redis = Redis::new(redis);

    let grpc_client: GrpcAccountClient = GrpcAccountClient::new().await;

    let ledger_handler = LedgerHandler::new(db.clone(), redis.clone(), grpc_client);
    let rest = new_ledger_router(ledger_handler);

    let grpc = Server::builder()
        .add_service(LedgerServer::new(LedgerService::new(db.clone(), redis.clone())))
        .into_service();

    let service = MultiplexService::new(rest, grpc);

    let addr = SocketAddr::from((IpAddr::from_str(&GLOBAL_CONFIG.read().await.server.listen_ip).unwrap(), GLOBAL_CONFIG.read().await.server.listen_port));
    hyper::Server::bind(&addr)
        .serve(Shared::new(service))
        .with_graceful_shutdown(akasha::app::shutdown_signal())
        .await
        .unwrap();
}

