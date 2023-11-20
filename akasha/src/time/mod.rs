use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, TimeZone};

pub fn timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap()
}

pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Local> {
    let length = timestamp.to_string().len();
    if length==10 {
        Local.timestamp_opt(timestamp, 0).unwrap()
    } else if length==13 {
        Local.timestamp_millis_opt(timestamp).unwrap()
    } else {
        Local.timestamp_millis_opt(0).unwrap()
    }
}
 
#[cfg(test)]
mod tests {
    use crate::time::timestamp_to_datetime;
    use chrono::{Local, TimeZone};

    #[test]
    fn test_timestamp_to_datetime() {
        assert_eq!(timestamp_to_datetime(1697698540), Local.datetime_from_str("2023-10-19 14:55:40","%Y-%m-%d%H:%M:%S").unwrap());
    }
}