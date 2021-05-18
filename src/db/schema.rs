table! {
    files (id) {
        id -> Int4,
        name -> Varchar,
        symmetric_key -> Varchar,
        users_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        public_key -> Varchar,
        shared_secret -> Varchar,
        shared_secret_salt -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    users,
);
