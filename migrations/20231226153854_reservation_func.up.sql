-- Add up migration script here
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
-- 这个函数，开发者可以决定放在应用程序层，或放在数据库层，都可以的，主要是看数据库是否容易实现，不必想太多
CREATE OR REPLACE FUNCTION rsvp.query(uid TEXT, rid TEXT, during TSTZRANGE) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
BEGIN
    -- if both are null, find all reservations within during
    IF uid IS NULL AND rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE during @> timespan;
    -- if user_id is null, find all reservations within during for the resource
    ELSIF uid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND during @> timespan;
    ELSIF rid IS NULL THEN
    -- if resource_id is null, find all reservations within during for the user
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND during @> timespan;
    ELSE
    -- if both set, find all reservations within during for the resource and user
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND resource_id = rid AND during @> timespan;
    END IF;
END;
$$ LANGUAGE plpgsql;
