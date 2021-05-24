use std::fs;
use std::str::FromStr;

use super::authentication::check_challenge;

use crate::db::{
    models::NewFile,
    repository::{
        FileRepository, PostgrSQLFileRepository, PostgrSQLUserRepository, UserRepository,
    },
};
use crate::errors::FileError;

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

pub fn post_file(
    username: &str,
    filename: &str,
    file_secret: &[u8],
    file_nonce: &[u8],
    challenge: &[u8],
    tag: &[u8],
) -> Result<(), FileError> {
    if let Err(_) = check_challenge(username, challenge, tag) {
        println!("challenge failed");
        return Err(FileError::UploadFailed);
    }

    let urepo = PostgrSQLUserRepository {};
    let frepo = PostgrSQLFileRepository {};
    let user = urepo.get_user(username).unwrap();

    let symmetric_key = base64::encode(file_secret);
    let nonce = base64::encode(file_nonce);

    let new_file = NewFile {
        name: filename,
        symmetric_key: symmetric_key.as_str(),
        nonce: nonce.as_str(),
        owner_id: user.id,
    };

    if let Err(_) = frepo.create_file(&new_file) {
        println!("file create failed");
        return Err(FileError::UploadFailed);
    }

    let user_vault = String::from("files/vault/") + &user.username + "/";

    println!("{}", user_vault);

    fs::rename(
        String::from("files/share/") + &filename,
        user_vault + filename,
    )
    .unwrap();

    Ok(())
}
