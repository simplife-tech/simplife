use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, NaiveDateTime, FixedOffset};

pub fn timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap()
 }

pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Local> {
   let timezone_east = FixedOffset::east_opt(0).unwrap();
   DateTime::<Local>::from_local(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), timezone_east)
}
 