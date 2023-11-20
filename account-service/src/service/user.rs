
use akasha::{crypto::sha3_512, db::Db, Context, redis::Redis, error::Error};

use crate::{dto::{login::{LoginReq, LoginReply}, error}, db::user::UserDao, handler::grpc::proto::v1::GetUidReply, cache::user::UserCache};

#[derive(Clone)]
pub struct UserService {
    user_dao: UserDao,
    user_cache: UserCache,
}

impl UserService {
    pub fn new(db: Db, redis: Redis) -> UserService {
        UserService {
            user_dao: UserDao::new(db),
            user_cache: UserCache::new(redis)
        }
    }

    pub async fn user_login(&self, ctx: &Context, req: LoginReq) -> Result<LoginReply, Error> {
        let password_hash = sha3_512(&req.password);
        match self.user_dao.find_user_by_mobile(ctx, &req.mobile).await {
            Ok(user) => {
                if user.password == password_hash {
                    match self.user_cache.set_session(ctx, &user.id).await {
                        Ok(session_id) => {
                            Ok(LoginReply{
                                uid: user.id,
                                access_key: session_id
                            })
                        },
                        Err(_) => {
                            Err(Error::NetWorkError)
                        }
                    }
                } else {
                    Err(error::LOGIN_FAILED)
                }
            },
            Err(_) => {
                Err(error::LOGIN_FAILED)
            }
        }
    }

    pub async fn get_uid(&self, ctx: &Context, access_key: String) -> Result<GetUidReply, Error> {
        let (uid, expires) = match self.user_cache.get_uid(ctx, &access_key).await {
            Ok((uid, expires)) => (uid, expires),
            Err(_) => return Err(Error::NetWorkError)
        };
        match uid {
            Some(uid) => {
                Ok(GetUidReply {uid, expires})
            },
            None => Err(error::NOT_LOGIN)
        }
    }

    pub async fn get_family_id(&self, ctx: &Context, uid: &i64) -> Result<i64, Error> {
        let family_id = match self.user_cache.get_family_id(ctx, uid).await {
            Ok(family_id) => {
                family_id
            },
            Err(_) => {
                None
            }
        };
        if let Some(family_id) = family_id {
            Ok(family_id)
        } else {
            match self.user_dao.get_family_id_by_uid(ctx, &uid).await {
                Ok(family_id) => {
                    let family_id = family_id.unwrap_or(-1);
                    let _ = self.user_cache.set_family_id(ctx, uid, &family_id).await;
                    Ok(family_id)
                }
                Err(_) => Err(Error::NetWorkError)
            }
        }
    }

}