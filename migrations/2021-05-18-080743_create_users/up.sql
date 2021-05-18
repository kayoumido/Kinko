create table users (
    id integer not null primary key,
    username varchar not null,
    password varchar not null,
    public_key varchar not null,
    shared_secret varchar not null,
    shared_secret_salt varchar not null
);