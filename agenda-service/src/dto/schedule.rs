use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetScheduleReq {
    pub start_time: i64,
    pub end_time: i64,
}
