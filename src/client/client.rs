pub mod crypto;

use crate::{errors::AuthError, server};

pub fn login(username: &str, passwd: &str) -> Result<(Vec<String>, Vec<u8>, Vec<u8>), AuthError> {
    let possible_salt_and_chall = server::authentication::get_salt_challenge(username);
    if let Err(_) = possible_salt_and_chall {
        println!("No user found");
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
        println!("Challenge failed");
        return Err(AuthError::LoginError);
    }

    let session_token = possible_session_token.unwrap();

    let session_tag = crypto::sign_token(session_token.as_slice(), shared_secret.as_slice());
    let files = server::files::get_my_files(username, session_token.as_ref(), session_tag.as_ref());

    // return the shared secret & session token
    Ok((files, shared_secret, session_token))
}

fn upload_file(filename: &str, username: &str, secret: &[u8], session_token: &[u8]) {}

fn download_file(filename: &str, username: &str, secret: &[u8], session_token: &[u8]) {}
