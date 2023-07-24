pub mod proto;

use redis::aio::ConnectionManager;
use sqlx::{MySql, Pool};
use tonic::{async_trait, Request, Response, Status, Code};

use crate::{db::Db, cache::Redis};

use self::proto::v1::{account_server::Account, AccessKey, GetUidReply, GetFamilyIdReply, Uid};

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
        let (_metadata, extensions, message) = request.into_parts();
        let oc = extensions.get::<akasha::opentelemetry::Context>().unwrap();

        if message.access_key.len()<1 {
            return Err(Status::new(Code::InvalidArgument, "access_key不合法"))
        }
        let (uid, expires) = match self.redis.get_uid(oc, &message.access_key).await {
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
    async fn get_family_id(&self, request: Request<Uid>) -> Result<Response<GetFamilyIdReply>, Status> {
        let (_metadata, extensions, message) = request.into_parts();
        let oc = extensions.get::<akasha::opentelemetry::Context>().unwrap();
        
        let family_id = match self.redis.get_family_id(oc, &message.uid).await {
            Ok(family_id) => {
                family_id
            },
            Err(err) => {
                log::error!("redis error! {}", err);
                None
            }
        };
        if let Some(family_id) = family_id {
            Ok(Response::new(GetFamilyIdReply {family_id}))
        } else {
            match self.db.get_family_id_by_uid(oc, &message.uid).await {
                Ok(family_id) => {
                    let family_id = family_id.unwrap_or(-1);
                    self.redis.set_family_id(oc, &message.uid, &family_id).await;
                    Ok(Response::new(GetFamilyIdReply {family_id}))
                }
                Err(_) => Err(Status::new(Code::Internal, "db异常"))
            }
        }
    }
}

