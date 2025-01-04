create table if not exists animal (
    name text not null primary key,
    created text not null default current_timestamp,
    score integer not null default 0,
    nickname text
);

