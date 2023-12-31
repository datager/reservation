-- Add up migration script here


-- 上文 proto 协议里定义的七个接口，目前只有 query() 需要实现一个 plpgsql，其他并没有必要单独实现 plpgsql


-- reservation change queue
CREATE TABLE rsvp.reservation_changes (
    id SERIAL NOT NULL,
    reservation_id UUID NOT NULL,
    op rsvp.reservation_update_type NOT NULL
);


-- trigger for add/update/delete a reservation
CREATE OR REPLACE FUNCTION rsvp.reservations_trigger() RETURNS TRIGGER AS $$
BEGIN
IF TG_OP = 'INSERT' THEN
    -- updete reservation_changes
    insert into rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'create');
ELSIF TG_OP = 'UPDATE' THEN
    -- if status changed, update reservation_changes
    IF NEW.status != OLD.status THEN
        insert into rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'update');
    END IF;
ELSIF TG_OP = 'DELETE' THEN
    -- update reservation_changes
    insert into rsvp.reservation_changes (reservation_id, op) VALUES (OLD.id, 'delete');
END IF;
-- notify a channel called reservation_update
NOTIFY reservation_update;
RETURN NULL; -- 因为是 AFTER 才触发的 trigger，所以 RETURN NULL 即可
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reservations_trigger AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservations FOR EACH ROW EXECUTE PROCEDURE rsvp.reservations_trigger();
