use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        Utc,
    )
}

pub fn convert_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as _,
    }
}
