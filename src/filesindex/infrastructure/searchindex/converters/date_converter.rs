use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::{time::serde::timestamp, DateTime};

pub fn unix_time_to_tantivy_datetime(timestamp: u64) -> DateTime {
    let system_time = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);

    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp_secs = duration_since_epoch.as_secs(); // Unix timestamp in seconds
    DateTime::from_timestamp_secs(timestamp_secs as i64)
}
