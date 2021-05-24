create table users (
    id serial primary key,
    username varchar not null,
    public_key varchar not null,
    shared_secret varchar not null,
    shared_secret_salt varchar not null
);

create table files (
    id serial primary key,
    name varchar not null,
    symmetric_key varchar not null,
    content_nonce varchar not null,
    name_nonce varchar not null,
    owner_id integer not null,

    constraint fk_ownership
      foreign key(owner_id) 
	  references users(id)
);

insert into users (username, public_key, shared_secret, shared_secret_salt) values (
    'AliceXOXO', 
    'BJnRg+bze7YdwWKfKODuwewCqdvCLxEiMbgjGORbwMK1xqRBcWpr8dILxJFlfaqefTBzInRzHtdysSHGAxoj7M8=', 
    'CpP6H2YRfRSStCBwmmB7eAI00rlZCeOKmNXlZ7DyIVY=', 
    'esM6awwBCndMbzph6SvwXggvsE0B38742FU8lokT2P8='
);

insert into files (name, symmetric_key, content_nonce, name_nonce, owner_id) values (
    'Fo_ZakTJWBWP9GUgVnoVaqWOU4EIdDUoAThk_R8=',
    'BJGNBiT6G4CdAkH4qdKccnb3F9XsVpf3sTvkleG8hf+6W7Q9e1Zf+SsTrjNqsi25RekJnbzZmBGtiQjW/We9DWLrV5givGjqQLaT2KLoFOXthrZsNSSLJeOdEqBi5NcV4mafKPjewa4UGVCFsFwBHxvYFeleVXnKH7O+Lja7lTs2',
    'yiwNSsCiJSBgnSFh',
    'TMaD2teGSROQpFeW',
    1
);