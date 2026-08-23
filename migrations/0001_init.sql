-- Add migration script here

-- TODO: identify by uuid, remove id 
create table players (
    id serial primary key not null,
    uuid uuid not null,
    name varchar(16),
    color_hex char(7),
    unique (uuid)
);

create table stats (
    player_id integer not null,
    timestamp timestamp not null,
    category varchar not null,
    name varchar not null,
    value integer not null,
    unique (player_id, timestamp, category, name)
);
