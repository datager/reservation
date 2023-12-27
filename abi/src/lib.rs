mod pb;
pub use pb::*;

use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        Utc,
    )
}
