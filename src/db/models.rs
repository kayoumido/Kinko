use super::schema::{files, users};

#[derive(Queryable, Debug, AsChangeset, PartialEq)]
// #[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub public_key: String,
    pub shared_secret: String,
    pub shared_secret_salt: String,
}

#[derive(Queryable, Debug, Associations)]
#[belongs_to(User foreign_key = "users_id")]
// #[table_name = "files"]
pub struct File {
    pub id: i32,
    pub name: String,
    pub symmetric_key: String,
    pub users_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub public_key: &'a str,
    pub shared_secret: &'a str,
    pub shared_secret_salt: &'a str,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub symmetric_key: &'a str,
    pub users_id: i32,
}
