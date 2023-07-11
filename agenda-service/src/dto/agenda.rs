use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddAgendaReq {
    pub title: String,
    pub content: String,
    pub access_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAgendaReq {
    pub pn: i64, // 页码
    pub ps: i64, // 页大小
    pub date_start: i64,
    pub date_end: i64,
    pub access_key: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteAgendaReq {
    pub access_key: String,
    pub id: i64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateAgendaReq {
    pub access_key: String,
    pub id: i64,
    pub title: String,
    pub content: String
}