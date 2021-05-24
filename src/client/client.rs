pub mod crypto;

use std::fs;

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
    let files = server::files::get_files(username, session_token.as_ref(), session_tag.as_ref());

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
    let (encrypted_file_name, file_secret, content_nonce, name_nonce) =
        crypto::encrypt_file(filename);

    let key = crypto::encrypt_key(&file_secret, pk);
    let session_tag = crypto::sign_token(session_token, shared_secret);

    let encrypted_file_to = String::from("files/share/") + encrypted_file_name.as_str();
    let encrypted_file_from = String::from("files/home/") + encrypted_file_name.as_str();

    fs::rename(encrypted_file_from, encrypted_file_to).unwrap();

    let res = server::files::post_file(
        username,
        encrypted_file_name.as_str(),
        base64::encode(key).as_str(),
        base64::encode(content_nonce).as_str(),
        base64::encode(name_nonce).as_str(),
        session_token,
        session_tag.as_ref(),
    );

    if let Err(_) = res {}
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

    // decode all the needed info for the file decryption
    let encrypted_file_secret = base64::decode(file.symmetric_key).unwrap();
    let file_nonce = base64::decode(file.content_nonce).unwrap();
    let name_nonce = base64::decode(file.name_nonce).unwrap();

    // decrypt the key used for encryption
    let file_secret = crypto::decrypt_key(encrypted_file_secret.as_ref(), sk);

    let encrypted_file_from = String::from("files/share/") + filename;

    let decrypted_file_name = crypto::decrypt_file(
        encrypted_file_from.as_str(),
        file_secret.as_ref(),
        file_nonce.as_ref(),
        name_nonce.as_ref(),
    );

    let decrypted_file_from = String::from("files/share/") + decrypted_file_name.as_str();
    let decrypted_file_to = String::from("files/home/") + decrypted_file_name.as_str();

    let _r = fs::remove_file(encrypted_file_from);

    fs::rename(decrypted_file_from, decrypted_file_to).unwrap();
}
