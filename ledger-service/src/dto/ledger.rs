use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddLedgerReq {
    pub amount: i64,
    pub comment: String,
    pub date: i64,
    pub access_key: String,
    pub clazz_1: String,
    pub clazz_2: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerDto {
    pub id: i64,
    pub amount: i64,
    pub comment: String,
    pub date: i64,
    pub clazz_1: String,
    pub clazz_2: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetLedgerReq {
    pub pn: i64, // 页码
    pub ps: i64, // 页大小
    pub date_start: i64,
    pub date_end: i64,
    pub access_key: String,
    pub kind: String // personal/family
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteLedgerReq {
    pub access_key: String,
    pub id: i64
}

