use super::connection;
use super::models::*;
use super::schema::files::dsl::*;
use super::schema::users::dsl::*;

use crate::errors::DBError;

use diesel::{insert_into, prelude::*};

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

pub trait FileRepository {
    fn create_file(&self, file: &NewFile) -> Result<(), DBError>;
    fn get_user_files(&self, ownerid: i32) -> Result<Vec<File>, DBError>;
    fn get_file(&self, ownerid: i32, filename: &str) -> Result<File, DBError>;
}

pub struct PostgrSQLFileRepository {}

impl FileRepository for PostgrSQLFileRepository {
    fn get_user_files(&self, ownerid: i32) -> Result<Vec<File>, DBError> {
        let conn = connection()?;

        let res = files.filter(owner_id.eq(ownerid)).load::<File>(&conn);

        if let Err(_) = res {
            Err(DBError::NoFiles)
        } else {
            Ok(res.unwrap())
        }
    }

    fn create_file(&self, file: &NewFile) -> Result<(), DBError> {
        let conn = connection()?;
        if let Err(_) = insert_into(files).values(file).execute(&conn) {
            return Err(DBError::FileCreationFailed);
        }

        Ok(())
    }

    fn get_file(&self, ownerid: i32, filename: &str) -> Result<File, DBError> {
        let conn = connection()?;
        let res = files
            .filter(owner_id.eq(ownerid).and(name.eq(filename)))
            .first::<File>(&conn);
        if let Err(_) = res {
            return Err(DBError::NoFiles);
        }

        Ok(res.unwrap())
    }
}
