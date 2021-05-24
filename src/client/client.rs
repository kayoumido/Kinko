pub mod crypto;

use std::fs;
use std::path::Path;

use crate::{errors::AuthError, server};

pub fn login(username: &str, passwd: &str) -> Result<(Vec<String>, Vec<u8>, Vec<u8>), AuthError> {
    let possible_salt_and_chall = server::authentication::get_salt_challenge(username);
    if let Err(_) = possible_salt_and_chall {
        return Err(AuthError::LoginError);
    }
    let (salt, challenge) = possible_salt_and_chall.unwrap();

    // (re)compute the shared secret based on the salt the server returned
    let shared_secret = crypto::compute_shared_secret(passwd, &salt);

    // play the challenge-response game
    let chall_tag = crypto::sign_token(challenge.as_slice(), shared_secret.as_slice());
    let possible_session_token = server::authentication::check_challenge(
        username,
        challenge.as_slice(),
        chall_tag.as_slice(),
    );
    if let Err(_) = possible_session_token {
        return Err(AuthError::LoginError);
    }

    let session_token = possible_session_token.unwrap();

    let session_tag = crypto::sign_token(session_token.as_ref(), shared_secret.as_ref());
    let files = server::files::get_my_files(username, session_token.as_ref(), session_tag.as_ref());

    // return the shared secret & session token
    Ok((files, shared_secret, session_token))
}

pub fn upload_file(
    filename: &str,
    username: &str,
    shared_secret: &[u8],
    session_token: &[u8],
    pk: &[u8],
) {
    let (enc_name, file_secret, nonce) = crypto::encrypt_file(filename);

    let enc_file: &str = &(String::from("files/share/") + &enc_name);
    let enc_secret = crypto::encrypt_key(&file_secret, pk);
    let session_tag = crypto::sign_token(session_token, shared_secret);

    fs::rename(String::from("files/home/") + &enc_name, enc_file).unwrap();

    let enc_name = Path::new(&enc_file).file_name().unwrap().to_str().unwrap();

    if let Err(_) = server::files::post_file(
        username,
        enc_name,
        enc_secret.as_ref(),
        nonce.as_ref(),
        session_token,
        session_tag.as_ref(),
    ) {}
}

fn download_file(filename: &str, username: &str, secret: &[u8], session_token: &[u8]) {}
