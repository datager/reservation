mod manager;

use async_trait::async_trait;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: sqlx::PgPool,
}

#[async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    /// change reservation status (if current status is `Pending`, change to `Confirmed`)
    async fn change_status(
        &self,
        rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, abi::Error>;
    /// update note
    async fn update_note(
        &self,
        rsvp: abi::Reservation,
        note: String,
    ) -> Result<abi::Reservation, abi::Error>;
    /// delete reservation
    async fn delete(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;
    /// query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error>;
}
