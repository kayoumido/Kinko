use super::schema::{files, users};

#[derive(Queryable, Debug, AsChangeset, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub public_key: String,
    pub shared_secret: String,
    pub shared_secret_salt: String,
}

#[derive(Queryable, Debug, Associations)]
#[belongs_to(User foreign_key = "owner_id")]
pub struct File {
    pub id: i32,
    pub name: String,
    pub symmetric_key: String,
    pub content_nonce: String,
    pub name_nonce: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub public_key: &'a str,
    pub shared_secret: &'a str,
    pub shared_secret_salt: &'a str,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub symmetric_key: &'a str,
    pub content_nonce: &'a str,
    pub name_nonce: &'a str,
    pub owner_id: i32,
}
