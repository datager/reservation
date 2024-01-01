use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;

use crate::{convert_to_timestamp, ReservationQuery, ReservationStatus, Validator};

use super::{get_timespan, validate_range};

impl ReservationQuery {
    pub fn new(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        status: ReservationStatus,
        page: i32,
        desc: bool,
        page_size: i32,
    ) -> Self {
        Self {
            user_id: uid.into(),
            resource_id: rid.into(),
            start: Some(convert_to_timestamp(start)),
            end: Some(convert_to_timestamp(end)),
            status: status as i32,
            page,
            desc,
            page_size,
        }
    }
    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}

impl Validator for ReservationQuery {
    fn validate(&self) -> Result<(), crate::Error> {
        validate_range(self.start.as_ref(), self.end.as_ref())?;
        Ok(())
    }
}
