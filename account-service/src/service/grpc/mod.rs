pub mod proto;
use redis::aio::ConnectionManager;
use sqlx::{MySql, Pool};
use tonic::{async_trait, Request, Response, Status, Code};

use crate::{db::Db, cache::Redis};

use self::proto::v1::{account_server::Account, AccessKey, GetUidReply};

pub struct AccountService {
    redis: Redis,
    db: Db,
}

impl AccountService {
    pub fn new(pool: Pool<MySql>, redis: ConnectionManager) -> AccountService {
        Self { db: Db::new(pool), redis: Redis::new(redis) }
    }
}

#[async_trait]
impl Account for AccountService {
    async fn get_uid(&self, request: Request<AccessKey>) -> Result<Response<GetUidReply>, Status> {
        let r = request.into_inner();
        if r.access_key.len()<1 {
            return Err(Status::new(Code::InvalidArgument, "access_key不合法"))
        }
        let (uid, expires) = match self.redis.get_uid(&r.access_key).await {
            Ok((uid, expires)) => (uid, expires),
            Err(_) => return Err(Status::new(Code::Internal, "redis异常"))
        };
        match uid {
            Some(uid) => {
                Ok(Response::new(GetUidReply {uid, expires}))
            },
            None => Err(Status::new(Code::NotFound, "未登录"))
        }
    }
}

