use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, Row, PgPool, types::Uuid};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        let status = abi::ReservationStatus::try_from(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let start = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        // generate a insert sql for the reservation
        // execute the sql
        let id: Uuid = sqlx::query(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id")
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?.get(0);

        rsvp.id = id.to_string();

        Ok(rsvp)
    }

    async fn change_status(
        &self,
        _rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _rsvp: abi::Reservation,
        _note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, _rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}


impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self{pool}
    }
}
#[cfg(test)]
mod tests {
    use abi::convert_to_timestamp;
    use chrono::FixedOffset;

    use super::*;

    #[test]
    fn a() {
        let start: DateTime<FixedOffset> = "2022-12-25T15:00:00-0700".parse().unwrap();
        println!("{:?}", start);
    }
    
    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2022-12-25T15:00:00-0700".parse().unwrap();
        let end: DateTime<FixedOffset> = "2022-12-28T12:00:00-0700".parse().unwrap();
        let rsvp = abi::Reservation{
            id: "".to_string(),
            user_id: "tyr".to_string(),
            resource_id: "ocean-view-room-713".to_string(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            note: "I'll arrive at 3pm, Please help to upgrade to execuitive room if possible".to_string(),
            status: abi::ReservationStatus::Pending as i32,
        };
        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(rsvp.id != "");
    }
}

