pub mod proto;

use akasha::{db::Db, redis::Redis, Context};
use tonic::{async_trait, Request, Response, Status, Code};

use crate::service::UserService;

use self::proto::v1::{account_server::Account, AccessKey, GetUidReply, GetFamilyIdReply, Uid};

pub struct AccountService {
    pub user_service: UserService
}

impl AccountService {
    pub fn new(db: Db, redis: Redis) -> AccountService {
        let user_service = UserService::new(db, redis);
        Self { user_service } 
    }
}

#[async_trait]
impl Account for AccountService {
    async fn get_uid(&self, request: Request<AccessKey>) -> Result<Response<GetUidReply>, Status> {
        let ctx = Context::new_from_tonic_request(&request);
        let message = request.into_inner();
        if message.access_key.len()<1 {
            return Err(Status::new(Code::InvalidArgument, "参数错误!"))
        }
        match self.user_service.get_uid(&ctx, message.access_key).await {
            Ok(reply) => {
                Ok(Response::new(reply))
            },
            Err(err) => {
                Err(Status::internal(err.to_string()))
            }
        }
    }

    async fn get_family_id(&self, request: Request<Uid>) -> Result<Response<GetFamilyIdReply>, Status> {
        let ctx = Context::new_from_tonic_request(&request);
        let message = request.into_inner();
        match self.user_service.get_family_id(&ctx, &message.uid).await {
            Ok(family_id) => {
                Ok(Response::new(GetFamilyIdReply{family_id}))
            },
            Err(err) => {
                Err(Status::internal(err.to_string()))
            }
        }
    }
}

