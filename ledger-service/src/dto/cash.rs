use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordCashReq {
    pub ammount: i64,
    pub date: i64,
    pub access_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetCashRecordReq {
    pub pn: i64, // 页码
    pub ps: i64, // 页大小
    pub date_start: i64,
    pub date_end: i64,
    pub access_key: String,
    pub family_id: i64
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteCashRecordReq {
    pub access_key: String,
    pub id: i64
}