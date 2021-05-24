table! {
    files (id) {
        id -> Int4,
        name -> Varchar,
        symmetric_key -> Varchar,
        content_nonce -> Varchar,
        name_nonce -> Varchar,
        owner_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        public_key -> Varchar,
        shared_secret -> Varchar,
        shared_secret_salt -> Varchar,
    }
}

joinable!(files -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    files,
    users,
);
