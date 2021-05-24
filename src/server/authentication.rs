use crate::db::repository::{PostgrSQLUserRepository, UserRepository};

use std::str;

use sodiumoxide::crypto::auth::{self, Key, Tag};
use sodiumoxide::randombytes;

pub fn get_salt_challenge(username: &str) -> Result<(Vec<u8>, Vec<u8>), ()> {
    let repo = PostgrSQLUserRepository {};
    let u = repo.get_user(username);

    if let Err(_) = u {
        return Err(());
    }

    let user = u.unwrap();
    let challenge = randombytes::randombytes(16);
    let salt = base64::decode(user.shared_secret_salt).unwrap();

    Ok((salt, challenge))
}

pub fn check_challenge(username: &str, challenge: &[u8], tag: &[u8]) -> Result<Vec<u8>, ()> {
    let tag = Tag::from_slice(tag);

    if tag == None {
        return Err(());
    }
    let tag = tag.unwrap();

    let repo = PostgrSQLUserRepository {};
    let u = repo.get_user(username);

    if let Err(_) = u {
        return Err(());
    }
    let user = u.unwrap();
    let decoded_key = base64::decode(user.shared_secret).unwrap();
    let shared_secret = Key::from_slice(decoded_key.as_slice()).unwrap();

    if !auth::verify(&tag, challenge, &shared_secret) {
        return Err(());
    }

    Ok(randombytes::randombytes(16))
}
