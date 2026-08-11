-- Add migration script here
create table players (
    id serial primary key not null,
    name varchar(16) not null,
    uuid uuid not null,
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
