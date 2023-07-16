use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;

pub fn timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap()
 }

pub fn timestamp_to_datetime(timestamp: i64) -> NaiveDateTime {
   NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap()
}
 