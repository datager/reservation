-- Add down migration script here
DROP TRIGGER reservation_trigger ON rsvp.reservations;
DROP FUNCTION rsvp.reservation_trigger();
DROP TABLE rsvp.reservations_changes CASCADE;