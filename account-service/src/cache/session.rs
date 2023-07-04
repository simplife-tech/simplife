use rand::{distributions::Alphanumeric, Rng};
use redis::{AsyncCommands, RedisError};

use crate::config::GLOBAL_CONFIG;

use super::Redis;


pub fn _session_key(id: &str) -> String {
    format!("SESSION:{}", id)
}

impl Redis {

    pub async fn set_session(&self, uid: &i64) -> Result<String, RedisError> {
        let mut manager = self.manager.clone();
        let access_key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let key = _session_key(&access_key);
        match manager.set_ex::<_, _, ()>(&key, uid, GLOBAL_CONFIG.read().await.service.session_expired_time.try_into().unwrap()).await {
            Ok(_) => return Ok(access_key),
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
    }

    pub async fn get_uid(&self, access_key: &String) -> Result<(Option<i64>, i64), RedisError>  {
        let mut manager = self.manager.clone();
        let key = _session_key(access_key);
        let uid: Option<i64> = match manager.get(&key).await {
            Ok(uid) => uid,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
        match uid {
            Some(uid) => {
                match manager.expire::<_, ()>(&key, GLOBAL_CONFIG.read().await.service.session_expired_time.try_into().unwrap()).await {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                };
                let expires: i64 = match manager.ttl(&key).await {
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

}