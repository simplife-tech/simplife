use akasha::{multiplex_service::MultiplexService, db::db_connect};
use app_state::AppState;
use axum::{routing::{post, get}, Router};
use cache::Redis;
use config::GLOBAL_CONFIG;
use db::Db;
use service::{grpc::{LedgerService, client::GrpcClient}, ledger::{add_ledger, ledger_list, delete_ledger}, cash::{record_cash, cash_record_list, delete_cash_record}};
use std::{net::{SocketAddr, IpAddr}, str::FromStr};
use tower::make::Shared;
use crate::service::grpc::proto::v1::ledger_server::LedgerServer;
mod db;
mod app_state;
mod service;
use tonic::transport::{Server, Endpoint};
use hyper;
use clap::Parser;
mod config;
mod cache;
mod dto;
mod strings;
#[macro_use]
extern crate lazy_static; 

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
    let channel = Endpoint::from_str(&args.account_service).unwrap().connect().await.unwrap();

    let app_state = AppState {db: Db::new(pool.clone()), redis: Redis::new(redis.clone()), grpc_client: GrpcClient::new(channel)};


    let rest = Router::new()
        .route("/ledger/add", post(add_ledger))
        .route("/ledger/list", get(ledger_list))
        .route("/ledger/delete", post(delete_ledger))
        .route("/cash/add", post(record_cash))
        .route("/cash/list", get(cash_record_list))
        .route("/cash/delete", post(delete_cash_record))
        .layer(axum::middleware::from_fn(akasha::app::trace_http))
        .with_state(app_state)
        ;
    let grpc = Server::builder()
        .add_service(LedgerServer::new(LedgerService::new(pool.clone(), redis.clone())))
        .into_service();

    let service = MultiplexService::new(rest, grpc);

    let addr = SocketAddr::from((IpAddr::from_str(&GLOBAL_CONFIG.read().await.server.listen_ip).unwrap(), GLOBAL_CONFIG.read().await.server.listen_port));
    hyper::Server::bind(&addr)
        .serve(Shared::new(service))
        .with_graceful_shutdown(akasha::app::shutdown_signal())
        .await
        .unwrap();
}

