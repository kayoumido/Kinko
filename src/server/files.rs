use std::str::FromStr;

use super::authentication::check_challenge;
use crate::db::repository::{
    FileRepository, PostgrSQLFileRepository, PostgrSQLUserRepository, UserRepository,
};

pub fn get_my_files(username: &str, challenge: &[u8], tag: &[u8]) -> Vec<String> {
    if let Err(_) = check_challenge(username, challenge, tag) {
        return Vec::<String>::new();
    }

    let urepo = PostgrSQLUserRepository {};
    let frepo = PostgrSQLFileRepository {};

    let user = urepo.get_user(username).unwrap();
    let files = frepo.get_user_files(user.id);

    if let Err(_) = files {
        return Vec::<String>::new();
    }
    let files = files.unwrap();
    let filenames: Vec<&str> = files.iter().map(|file| file.name.as_ref()).collect();

    filenames
        .iter()
        .map(|f| String::from_str(*f).unwrap())
        .collect()
}
