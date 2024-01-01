-- Add up migration script here
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
-- 这个函数，开发者可以决定放在应用程序层，或放在数据库层，都可以的，主要是看数据库是否容易实现，不必想太多
-- select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")');
-- select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")', 1, true);
-- select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")', 1, false, 2);
-- select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")', 2, false, 2);
-- select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")', 3, false, 2);
-- explain select * from rsvp.query('y', NULL, '("2022-01-01", "2023-01-01")', 1, false, 2);
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('y', 'room-422', '("2022-11-16", "2022-11-17")');
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('y', 'room-422', '("2022-11-17", "2022-11-18")');
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('y', 'room-421', '("2022-11-17", "2022-11-18")');
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('alice', 'room-421', '("2022-11-18", "2022-11-19")');
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('y', 'room-421', '("2023-1-17", "2023-1-18")');
-- insert into rsvp.reservations (user_id, resource_id, timespan) values ('y', 'room-421', '("2022-11-01", "2022-11-5")');

CREATE OR REPLACE FUNCTION rsvp.query(
    uid TEXT, 
    rid TEXT, 
    during TSTZRANGE, 
    page INTEGER DEFAULT 1,
    is_desc bool DEFAULT FALSE,
    page_size INTEGER DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
DECLARE
    _sql TEXT;
BEGIN
    -- format the query based on parameters
    _sql := format(
        'SELECT * FROM rsvp.reservations WHERE %L @> timespan AND %s ORDER BY LOWER(timespan) %s LIMIT %L::INTEGER OFFSET %L::INTEGER', 
        during,
        CASE
            WHEN uid IS NULL AND rid IS NULL THEN 'TRUE'
            WHEN uid IS NULL THEN 'resource_id = ' || quote_literal(rid)
            WHEN rid IS NULL THEN 'user_id = ' || quote_literal(uid)
            ELSE 'user_id = ' || quote_literal(uid) || ' AND resource_id = ' || quote_literal(rid)
        END,
        CASE 
            WHEN is_desc THEN 'DESC'
            ELSE 'ASC'
        END,
        page_size,
        (page - 1) * page_size
    );

    -- log the sql
    RAISE NOTICE '%', _sql;

    -- execute the query
    RETURN QUERY EXECUTE _sql;
END;
$$ LANGUAGE plpgsql;
