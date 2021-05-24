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
    let (enc_filename, file_secret, nonce) = crypto::encrypt_file(filename);

    let enc_file: &str = &(String::from("files/share/") + &enc_filename);
    let enc_secret = crypto::encrypt_key(&file_secret, pk);
    let session_tag = crypto::sign_token(session_token, shared_secret);

    fs::rename(String::from("files/home/") + &enc_filename, enc_file).unwrap();

    let enc_filename = Path::new(&enc_file).file_name().unwrap().to_str().unwrap();

    if let Err(_) = server::files::post_file(
        username,
        enc_filename,
        enc_secret.as_ref(),
        nonce.as_ref(),
        session_token,
        session_tag.as_ref(),
    ) {}
}

pub fn download_file(
    filename: &str,
    username: &str,
    shared_secret: &[u8],
    session_token: &[u8],
    sk: &[u8],
) {
    let session_tag = crypto::sign_token(session_token, shared_secret);

    let file = server::files::get_file(username, filename, session_token, session_tag.as_ref());

    if let Err(_) = file {}
    let file = file.unwrap();

    let enc_file_secret = base64::decode(file.symmetric_key).unwrap();
    let file_nonce = base64::decode(file.nonce).unwrap();
    let file_secret = crypto::decrypt_key(enc_file_secret.as_ref(), sk);

    let enc_file_path: &str = &(String::from("files/share/") + filename);
    let dec_file_path = String::from("files/home/") + filename + ".unlocked";
    crypto::decrypt_file(&enc_file_path, file_secret.as_ref(), file_nonce.as_ref());

    let _r = fs::remove_file(enc_file_path);

    fs::rename(enc_file_path.to_string() + ".unlocked", dec_file_path).unwrap();
}
