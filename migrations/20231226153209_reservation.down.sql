-- Add down migration script here
DROP TABLE IF EXISTS rsvp.reservations CASCADE;
DROP TYPE IF EXISTS rsvp.reservation_update_type CASCADE;
DROP TYPE IF EXISTS rsvp.reservation_status CASCADE;