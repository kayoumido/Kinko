use super::connection;
use super::models::*;
use super::schema::users::dsl::*;
use diesel::prelude::*;

use crate::errors::DBError;

pub trait UserRepository {
    /// Try and get a user from the storage
    /// if the wanted user doesn't exist, an error is returned
    ///
    /// # Arguments
    ///
    /// * `e` - email of the user to retrieve
    ///
    fn get_user(&self, uname: &str) -> Result<User, DBError>;
}

pub struct PostgrSQLUserRepository {}

/// Implementation of the `UserRepository` with PostgreSQL as a storage
impl UserRepository for PostgrSQLUserRepository {
    fn get_user(&self, usrname: &str) -> Result<User, DBError> {
        let conn = connection()?;

        let res = users.filter(username.eq(usrname)).first::<User>(&conn);

        if let Err(_) = res {
            Err(DBError::UserNotFound)
        } else {
            Ok(res.unwrap())
        }
    }
}
