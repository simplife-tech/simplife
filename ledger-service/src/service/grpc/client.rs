use akasha::{opentelemetry::{global, Context}, grpc::{MetadataExtractor, MetadataInjector}};
use tonic::transport::Channel;

use super::proto::account_service::{account_client::AccountClient, AccessKey, Uid};



#[derive(Clone)]
pub struct GrpcClient {
    account_service: AccountClient<Channel>
}

impl GrpcClient {
    pub fn new(channel: Channel) -> GrpcClient {
        Self { account_service: AccountClient::new(channel.clone()) }
    }

    pub async fn get_uid(&self, oc: &Context, access_key: &str) -> Result<i64, tonic::Status> {
        let mut request = tonic::Request::new(AccessKey {
            access_key: access_key.to_string()
        });
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(oc, &mut MetadataInjector(request.metadata_mut()))
        });
        let response = self.account_service.clone().get_uid(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().uid),
            Err(err) => Err(err),
        }
    }

    pub async fn get_family_id(&self, oc: &Context, uid: &i64) -> Result<i64, tonic::Status> {
        let mut request = tonic::Request::new(Uid {
            uid: *uid
        });
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(oc, &mut MetadataInjector(request.metadata_mut()))
        });
        let response = self.account_service.clone().get_family_id(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().family_id),
            Err(err) => Err(err),
        }
    }
}