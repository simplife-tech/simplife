use std::str::FromStr;

use opentelemetry::global;
use tonic::transport::{Channel, Endpoint};

use crate::MetadataInjector;

use super::proto::account_service::{account_client::AccountClient, Uid, AccessKey, GetUidReply};


#[derive(Clone)]
pub struct GrpcAccountClient {
    account_service: AccountClient<Channel>
}

impl GrpcAccountClient {
    pub async fn new() -> GrpcAccountClient {
        let channel = Endpoint::from_str("http://account-service:27001").unwrap().connect().await.unwrap();
        Self { account_service: AccountClient::new(channel) }
    }

    pub async fn get_uid(&self, ctx: &akasha::Context, access_key: &str) -> Result<GetUidReply, tonic::Status> {
        let mut request = tonic::Request::new(AccessKey {
            access_key: access_key.to_string()
        });
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&ctx.opentelemetry_context, &mut MetadataInjector(request.metadata_mut()))
        });
        let response = self.account_service.clone().get_uid(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().clone()),
            Err(err) => return Err(err),
        }
    }

    pub async fn get_family_id(&self, ctx: &akasha::Context, uid: &i64) -> Result<i64, tonic::Status> {
        let mut request = tonic::Request::new(Uid {
            uid: *uid
        });
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&ctx.opentelemetry_context, &mut MetadataInjector(request.metadata_mut()))
        });
        let response = self.account_service.clone().get_family_id(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().family_id),
            Err(err) => Err(err),
        }
    }
}