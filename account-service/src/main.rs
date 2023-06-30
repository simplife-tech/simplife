use akasha::{multiplex_service::MultiplexService, db::db_connect};
use app_state::AppState;
use axum::{routing::{ post}, Router};
use cache::Redis;
use config::{GLOBAL_CONFIG};
use db::Db;
use service::{grpc::{AccountService, proto::v1::account_server::AccountServer}, login::user_login};
use std::{net::{SocketAddr, IpAddr}, str::FromStr};
use tower::{make::Shared};
mod service;
mod dto;
mod db;
mod app_state;
use tonic::transport::Server;
use hyper;
use clap::Parser;
mod config;
mod cache;
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

	#[arg(long, default_value_t = 300)]
    session_expired_time: i32,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  {
    let mut config = GLOBAL_CONFIG.write().await;
    config.db.url = args.db;
    config.db.max_connections = args.db_max_connections;
    config.server.listen_ip = args.listen_ip;
    config.server.listen_port = args.listen_port;
	  config.service.session_expired_time = args.session_expired_time;
  }

    let pool = db_connect(&GLOBAL_CONFIG.read().await.db.url, GLOBAL_CONFIG.read().await.db.max_connections).await.unwrap();
    let redis = redis::Client::open(args.redis).unwrap();
  	let app_state = AppState {db: Db::new(pool.clone()), redis: Redis::new(redis.clone())};

	let rest = Router::new()
        .route("/login", post(user_login))
        .with_state(app_state)
        ;
    let grpc = Server::builder()
        .add_service(AccountServer::new(AccountService::new(pool.clone(), redis.clone())))
        .into_service();

    let service = MultiplexService::new(rest, grpc);

    let addr = SocketAddr::from((IpAddr::from_str(&GLOBAL_CONFIG.read().await.server.listen_ip).unwrap(), GLOBAL_CONFIG.read().await.server.listen_port));
    hyper::Server::bind(&addr)
        .serve(Shared::new(service))
        .with_graceful_shutdown(akasha::app::shutdown_signal())
        .await
        .unwrap();
}

