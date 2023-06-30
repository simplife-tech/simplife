use rand::{distributions::Alphanumeric, Rng};
use redis::{AsyncCommands};

use crate::config::GLOBAL_CONFIG;

use super::Redis;


pub fn _session_key(id: &str) -> String {
    format!("SESSION:{}", id)
}

impl Redis {

    pub async fn set_session(&self, uid: &i64) -> String {
        let mut con = self.client.get_async_connection().await.unwrap();
        let access_key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let key = _session_key(&access_key);
        let _: () = con.set_ex(&key, uid, GLOBAL_CONFIG.read().await.service.session_expired_time.try_into().unwrap()).await.unwrap();
        return access_key;
    }

    pub async fn get_uid(&self, access_key: &String) -> (Option<i64>, i64)  {
        let mut con = self.client.get_async_connection().await.unwrap();
        let key = _session_key(access_key);
        let uid: Option<i64> = con.get(&key).await.unwrap();
        match uid {
            Some(uid) => {
                let _: () = con.expire(&key, GLOBAL_CONFIG.read().await.service.session_expired_time.try_into().unwrap()).await.unwrap();
                let expires: i64 = con.ttl(&key).await.unwrap();
                (Some(uid), akasha::time::timestamp() + expires)
            },
            None => (None, 0),
        }
    }

}