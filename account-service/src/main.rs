use akasha::{multiplex_service::MultiplexService, db::{db_connect, Db}};
use config::GLOBAL_CONFIG;
use handler::{grpc::{AccountService, proto::v1::account_server::AccountServer}, http::{login::LoginHandler, new_router}};
use core::panic;
use std::{net::{SocketAddr, IpAddr}, str::FromStr};
use tower::make::Shared;
use akasha::redis::Redis;
mod handler;
mod dto;
mod db;
use tonic::transport::Server;
use hyper;
use clap::Parser;
mod config;
mod cache;
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

	#[arg(long, default_value_t = 60*60)]
    session_expired_time: i32,

    #[arg(long, default_value_t = String::from("api.simplife.tech:6831"))]
    trace_endpoint: String,

    #[arg(long, default_value_t = String::from("test"))]
    service_version: String,
}



#[tokio::main]
async fn main() {
    akasha::log::init_config(log::LevelFilter::Info);
    let args = Args::parse();
    akasha::app::init_tracer(args.trace_endpoint, "account.service".to_string(), args.service_version);


    {
        let mut config = GLOBAL_CONFIG.write().await;
        config.db.url = args.db;
        config.db.max_connections = args.db_max_connections;
        config.server.listen_ip = args.listen_ip;
        config.server.listen_port = args.listen_port;
        config.service.session_expired_time = args.session_expired_time;
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
	let login_handler = LoginHandler::new(db.clone(), redis.clone());
    let rest = new_router(login_handler);
        
    let grpc = Server::builder()
        .layer(akasha::grpc::GrpcOpentelemetryLayer)
        .add_service(AccountServer::new(AccountService::new(db.clone(), redis.clone())))
        .into_service();

    let service = MultiplexService::new(rest, grpc);
    let listen_ip = match IpAddr::from_str(&GLOBAL_CONFIG.read().await.server.listen_ip) {
        Ok(listen_ip) => listen_ip,
        Err(err) => {
            panic!("parse listen_ip error! {}", err)
        }
    };
    let addr = SocketAddr::from((listen_ip, GLOBAL_CONFIG.read().await.server.listen_port));
    hyper::Server::bind(&addr)
        .serve(Shared::new(service))
        .with_graceful_shutdown(akasha::app::shutdown_signal())
        .await
        .unwrap();
}

