-- Add up migration script here
CREATE TYPE rsvp.reservation_status AS ENUM ('unknown', 'pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM ('unknown', 'create', 'update', 'delete');
CREATE TABLE rsvp.reservations (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    status rsvp.reservation_status NOT NULL DEFAULT 'pending', -- 自定义类型
    resource_id TEXT NOT NULL, -- 外部系统传入, 无需 uuid, 无需 FOREIGN KEY CONSTRAINT
    timespan TSTZRANGE NOT NULL, -- 用区间描述
    note TEXT,
    CONSTRAINT reservations_pkey PRIMARY KEY(id),
    CONSTRAINT reservations_conflict EXCLUDE USING GIST (resource_id WITH =, timespan WITH &&) -- EXCLUDE CONSTRAINT, 不能存在 resource_id 相同, 且 timespan 区间有交集的两行
);
CREATE INDEX reservations_resource_id_idx ON rsvp.reservations (resource_id);
CREATE INDEX reservations_user_id_idx ON rsvp.reservations (user_id);