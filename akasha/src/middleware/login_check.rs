use std::str::FromStr;

use axum::{middleware::Next, response::IntoResponse};
use http::{Request, StatusCode};
use opentelemetry::global;
use tonic::transport::Endpoint;

use crate::{Context, error::Error, proto::account_service::{account_client::AccountClient, AccessKey}, grpc::MetadataInjector};

pub async fn login_check<B>(
    mut req: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let access_key = match req.headers().get("access_key") {
        Some(token) => token.to_str().unwrap(),
        None => return Ok(Error::NotLogin.into_response())
    };
    let channel = Endpoint::from_str("http://account-service:27001").unwrap().connect().await.unwrap();
    let account_client = AccountClient::new(channel);
    let mut request = tonic::Request::new(AccessKey {
        access_key: access_key.to_string()
    });
    let ctx = req.extensions_mut().get_mut::<Context>().unwrap();
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&ctx.opentelemetry_context, &mut MetadataInjector(request.metadata_mut()))
    });
    let response = account_client.clone().get_uid(request).await;
    let uid = match response {
        Ok(reply) => reply.get_ref().uid,
        Err(_) => return Ok(Error::NotLogin.into_response()),
    };
    ctx.keys_i64.insert("uid", uid);
    let res = next.run(req).await;
    
    Ok(res)
}