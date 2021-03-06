use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::{fs::File, io::BufRead, io::BufReader};

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

use ecies;

use sodiumoxide::crypto::auth;
use sodiumoxide::crypto::kdf;
use sodiumoxide::crypto::pwhash::{self, Salt};
use sodiumoxide::crypto::secretbox;
use sodiumoxide::randombytes;

///
pub fn compute_shared_secret(passwd: &str, salt: &[u8]) -> Vec<u8> {
    let salt = Salt::from_slice(salt).unwrap();

    let mut key = [0u8; secretbox::KEYBYTES];

    pwhash::derive_key(
        &mut key,
        passwd.as_bytes(),
        &salt,
        pwhash::OPSLIMIT_INTERACTIVE,
        pwhash::MEMLIMIT_INTERACTIVE,
    )
    .unwrap()
    .to_vec()
}

///
pub fn sign_token(token: &[u8], secret: &[u8]) -> Vec<u8> {
    let mut state = auth::State::init(secret);
    state.update(token);
    state.finalize().as_ref().to_vec()
}

pub fn encrypt_file(filepath: &str) -> (String, Vec<u8>, Vec<u8>, Vec<u8>) {
    // open the file and get it's contents
    let path = Path::new(filepath);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't read file {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let contents: String = reader.lines().map(|l| l.unwrap()).collect();

    // generate a symetric key for the encryption
    let key = kdf::gen_key();
    // encrypt the contents of the file
    let (cipher_content, content_nonce) = _encrypt(&contents.as_ref(), key.as_ref());

    // now we can write the encrypted contents in a new file

    // get the filename and the parent
    let fileanme = path.file_name().unwrap().to_str().unwrap();
    let parent = path.parent().unwrap().to_str().unwrap();

    // encrypt the filename using the same key
    let (cipher_name, name_nonce) = _encrypt(fileanme.as_ref(), key.as_ref());

    let display_name = base64::encode_config(cipher_name.as_slice(), base64::URL_SAFE);
    let encrypted_path = String::from(parent) + "/" + display_name.as_str();

    let enc_path = Path::new(&encrypted_path);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&enc_path) {
        Err(why) => panic!("couldn't create: {}, {:?}", why, enc_path),
        Ok(file) => file,
    };

    match file.write_all(cipher_content.as_ref()) {
        Err(why) => panic!("couldn't write to: {}", why),
        Ok(_) => (),
    }

    // return the key underwhich the file was encrypted and the nounce used
    (
        display_name,
        key.as_ref().to_vec(),
        content_nonce,
        name_nonce,
    )
}

pub fn decrypt_file(filename: &str, key: &[u8], nonce: &[u8], name_nonce: &[u8]) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't read file {}", why),
        Ok(file) => file,
    };

    // read the whole file
    let mut contents = Vec::new();
    if let Err(why) = file.read_to_end(&mut contents) {
        panic!("Couldn't read encrypted file: {}", why);
    }

    let decrypted = _decrypt(contents.as_ref(), key, nonce);
    let plaintext = str::from_utf8(decrypted.as_slice()).unwrap();

    let filename = path.file_name().unwrap().to_str().unwrap();
    let parent = path.parent().unwrap().to_str().unwrap();

    // decrypt de filename
    let encrypted_filename = base64::decode_config(filename, base64::URL_SAFE).unwrap();
    let og_name = _decrypt(encrypted_filename.as_ref(), key, name_nonce);
    let new_name = String::from("dec_") + str::from_utf8(og_name.as_ref()).unwrap();

    let decrypted_file_to = String::from(parent) + "/" + new_name.as_str();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(decrypted_file_to.as_str()) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
    };

    match file.write_all(plaintext.as_ref()) {
        Err(why) => panic!("couldn't write to: {}", why),
        Ok(_) => (),
    }

    new_name
}

pub fn encrypt_key(secret: &[u8], pk: &[u8]) -> Vec<u8> {
    let enc_key = ecies::encrypt(pk, secret);

    enc_key.unwrap()
}

pub fn decrypt_key(enc_secret: &[u8], pk: &[u8]) -> Vec<u8> {
    let key = ecies::decrypt(pk, enc_secret);

    key.unwrap()
}

fn _encrypt(plaintext: &[u8], key: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // convert key to a GenericArray
    let sym_key = GenericArray::clone_from_slice(key);
    // create new cipher for encryption
    let cipher = Aes256Gcm::new(&sym_key);

    // generate random nonce
    let n = randombytes::randombytes(12);
    let nonce = GenericArray::from_slice(n.as_ref());

    // encrypt the contents of the file
    let ciphertext = cipher.encrypt(nonce, plaintext).unwrap();

    (ciphertext, n)
}

pub fn _decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    // convert key & nonce to a GenericArray
    let sym_key = GenericArray::clone_from_slice(key);
    let nonce = GenericArray::from_slice(nonce);
    // create new cipher for encryption
    let cipher = Aes256Gcm::new(&sym_key);

    // decrypt the contents of the file
    cipher.decrypt(&nonce, ciphertext).unwrap()
}
