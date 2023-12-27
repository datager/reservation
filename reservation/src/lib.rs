mod error;

use async_trait::async_trait;
pub use error::ReservationError;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    /// change reservation status (if current status is `Pending`, change to `Confirmed`)
    async fn change_status(
        &self,
        rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError>;
    /// update note
    async fn update_note(
        &self,
        rsvp: abi::Reservation,
        note: String,
    ) -> Result<abi::Reservation, ReservationError>;
    /// delete reservation
    async fn delete(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError>;
}
