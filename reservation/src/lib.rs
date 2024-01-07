mod manager;

use abi::ReservationId;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    /// change reservation status (if current status is pending, change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;
    /// update note
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<(), abi::Error>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;
    /// query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error>;
    async fn filter(
        &self,
        filter: abi::ReservationFilter,
    ) -> Result<Vec<abi::Reservation>, abi::Error>;
}
