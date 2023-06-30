use std::time::{SystemTime, UNIX_EPOCH};

 pub fn timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap()
 }