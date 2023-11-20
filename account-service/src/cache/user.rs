use akasha::{Context, redis::Redis};
use rand::{distributions::Alphanumeric, Rng};
use redis::{AsyncCommands, RedisError};
use crate::config::GLOBAL_CONFIG;

#[derive(Clone)]
pub struct UserCache {
    redis: Redis
}

pub fn _session_key(id: &str) -> String {
    format!("SESSION:{}", id)
}

pub fn _family_id_key(uid: &i64) -> String {
    format!("FAMILY_ID:{}", uid.to_string())
}

impl UserCache {
    pub fn new(redis: Redis) -> UserCache {
        UserCache {
            redis
        }
    }

    pub async fn set_session(&self, ctx: &Context, uid: &i64) -> Result<String, RedisError> {
        let mut manager = self.redis.manager.clone();
        let access_key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let key = _session_key(&access_key);
        match set_ex!(ctx, manager, &key, uid, GLOBAL_CONFIG.read().await.service.session_expired_time).await {
            Ok(_) => return Ok(access_key),
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
    }

    pub async fn get_uid(&self, ctx: &Context, access_key: &String) -> Result<(Option<i64>, i64), RedisError>  {
        let mut manager = self.redis.manager.clone();
        let key = _session_key(access_key);
        let uid: Option<i64> = match get!(ctx, manager, &key).await {
            Ok(uid) => uid,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
        match uid {
            Some(uid) => {
                match expire!(ctx, manager, &key, GLOBAL_CONFIG.read().await.service.session_expired_time).await {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                };
                let expires: i64 = match ttl!(ctx, manager, &key).await {
                    Ok(expires) => expires,
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                };
                Ok((Some(uid), akasha::time::timestamp() + expires))
            },
            None => Ok((None, 0)),
        }
    }

    pub async fn set_family_id(&self, ctx: &Context, uid: &i64, family_id: &i64) -> Result<bool, RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _family_id_key(uid);
        if let Err(err) = set_ex!(ctx, manager, &key, family_id, 60*60*2).await {
            log::error!("redis error! {}", err);
            return Err(err)
        }
        return Ok(true)
    }

    pub async fn get_family_id(&self, ctx: &Context, uid: &i64) -> Result<Option<i64>, RedisError> {
        let mut manager = self.redis.manager.clone();
        let key = _family_id_key(uid);
        let family_id: Option<i64> = match get!(ctx, manager, &key).await {
            Ok(family_id) => family_id,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            },
        };
        return Ok(family_id)
    }

}
