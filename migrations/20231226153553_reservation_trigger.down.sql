-- Add down migration script here
DROP TRIGGER IF EXISTS reservation_trigger ON rsvp.reservations;
DROP FUNCTION IF EXISTS rsvp.reservation_trigger();
DROP TABLE IF EXISTS rsvp.reservations_changes CASCADE;