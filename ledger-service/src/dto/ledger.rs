use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerReq {
    pub ammount: i64,
    pub comment: String,
    pub date: i64,
    pub access_key: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerRsp {

}