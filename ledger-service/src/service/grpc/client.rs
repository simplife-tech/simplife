use akasha::request;
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

    pub async fn get_uid(&self, access_key: &str) -> Result<i64, tonic::Status> {
        let request = tonic::Request::new(AccessKey {
            access_key: access_key.to_string()
        });
        let response = self.account_service.clone().get_uid(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().uid),
            Err(err) => Err(err),
        }
    }

    pub async fn get_family_id(&self, uid: &i64) -> Result<i64, tonic::Status> {
        let request = tonic::Request::new(Uid {
            uid: *uid
        });
        let response = self.account_service.clone().get_family_id(request).await;
        match response {
            Ok(reply) => Ok(reply.get_ref().family_id),
            Err(err) => Err(err),
        }
    }
}