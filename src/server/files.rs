use std::fs;

use super::authentication::check_challenge;

use crate::db::models::{File, NewFile};
use crate::db::repository::{
    FileRepository, PostgrSQLFileRepository, PostgrSQLUserRepository, UserRepository,
};
use crate::errors::FileError;

pub fn get_files(username: &str, challenge: &[u8], tag: &[u8]) -> Vec<File> {
    if let Err(_) = check_challenge(username, challenge, tag) {
        return Vec::<File>::new();
    }

    let urepo = PostgrSQLUserRepository {};
    let frepo = PostgrSQLFileRepository {};

    let user = urepo.get_user(username).unwrap();
    let files = frepo.get_user_files(user.id);

    if let Err(_) = files {
        return Vec::<File>::new();
    }
    files.unwrap()
}

pub fn post_file(
    username: &str,
    filename: &str,
    symmetric_key: &str,
    content_nonce: &str,
    name_nonce: &str,
    challenge: &[u8],
    tag: &[u8],
) -> Result<(), FileError> {
    if let Err(_) = check_challenge(username, challenge, tag) {
        return Err(FileError::UploadFailed);
    }

    let urepo = PostgrSQLUserRepository {};
    let frepo = PostgrSQLFileRepository {};
    let user = urepo.get_user(username).unwrap();

    let new_file = NewFile {
        name: filename,
        symmetric_key,
        content_nonce,
        name_nonce,
        owner_id: user.id,
    };

    if let Err(_) = frepo.create_file(&new_file) {
        return Err(FileError::UploadFailed);
    }

    let user_vault = String::from("files/vault/") + user.username.as_str() + "/";

    fs::rename(
        String::from("files/share/") + &filename,
        user_vault + filename,
    )
    .unwrap();

    Ok(())
}

pub fn get_file(
    username: &str,
    filename: &str,
    challenge: &[u8],
    tag: &[u8],
) -> Result<File, FileError> {
    if let Err(_) = check_challenge(username, challenge, tag) {
        return Err(FileError::DownloadFailed);
    }

    let urepo = PostgrSQLUserRepository {};
    let frepo = PostgrSQLFileRepository {};
    let user = urepo.get_user(username).unwrap();

    let file = frepo.get_file(user.id, filename);

    if let Err(_) = file {
        return Err(FileError::DownloadFailed);
    }

    let user_vault = String::from("files/vault/") + user.username.as_str() + "/";
    fs::copy(
        user_vault + filename,
        String::from("files/share/") + &filename,
    )
    .unwrap();

    Ok(file.unwrap())
}
