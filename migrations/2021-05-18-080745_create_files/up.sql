create table files (
    id serial primary key,
    name varchar not null,
    symmetric_key varchar not null,
    nonce varchar not null,
    owner_id integer not null,

    constraint fk_ownership
      foreign key(owner_id) 
	  references users(id)
);