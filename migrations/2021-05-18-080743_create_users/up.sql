create table users (
    id serial primary key,
    username varchar not null,
    public_key varchar not null,
    shared_secret varchar not null,
    shared_secret_salt varchar not null
);