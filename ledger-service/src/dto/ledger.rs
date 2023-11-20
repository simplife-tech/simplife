use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddLedgerReq {
    pub amount: i64,
    pub comment: String,
    pub date: i64,
    pub clazz_1: i64,
    pub clazz_2: i64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerDto {
    pub id: i64,
    pub uid: i64,
    pub family_id: i64,
    pub amount: i64,
    pub comment: String,
    pub date: i64,
    pub clazz_1: i64,
    pub clazz_2: i64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerListReq {
    pub pn: i64, // 页码
    pub ps: i64, // 页大小
    pub date_start: i64,
    pub date_end: i64,
    pub kind: String // personal/family
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteLedgerReq {
    pub id: i64
}

