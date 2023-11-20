use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginReq {
    pub mobile: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginReply {
    pub uid: i64,
    pub access_key: String
}