-- Add migration script here

create table players (
    id serial primary key not null,
    name varchar(64), 
    color_hex char(7),
    unique(name) 
);

create table player_uuids (
    uuid uuid not null primary key, 
    player_id integer references players(id), 
);

create table stats (
    player_id integer not null references players(id),
    timestamp timestamp not null,
    category varchar not null,
    name varchar not null,
    value integer not null,
    unique (player_id, timestamp, category, name)
);
