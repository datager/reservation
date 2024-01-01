-- Add up migration script here
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
-- 这个函数，开发者可以决定放在应用程序层，或放在数据库层，都可以的，主要是看数据库是否容易实现，不必想太多
-- select * from rsvp.query('y', NULL, '("2023-01-01", "2024-01-01")');
CREATE OR REPLACE FUNCTION rsvp.query(
    uid TEXT, 
    rid TEXT, 
    during TSTZRANGE, 
    page INTEGER DEFAULT 1,
    page_size INTEGER DEFAULT 10,
    is_desc bool DEFAULT FALSE
) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
DECLARE
    _sql TEXT;
BEGIN
    -- format the query based on parameters
    _sql := format(
        'SELECT * FROM rsvp.reservations WHERE %L @> timespan AND %s ORDER BY LOWER(timespan) %s LIMIT %s OFFSET %s', 
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

    -- execute the query
    RETURN QUERY EXECUTE _sql;
END;
$$ LANGUAGE plpgsql;
