create table files (
    id integer not null primary key,
    name varchar not null,
    symmetric_key varchar not null,
    users_id integer not null
);