use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::{fs::File, io::BufRead, io::BufReader};

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

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

pub fn encrypt_file(filename: &str) -> (String, Vec<u8>, Vec<u8>) {
    // open the file and get it's contents
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't read file {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let contents: String = reader.lines().map(|l| l.unwrap()).collect();

    // generate a symetric key for the encryption
    let key = kdf::gen_key();
    let sym_key = GenericArray::clone_from_slice(key.as_ref());

    // create new cipher for encryption
    let cipher = Aes256Gcm::new(&sym_key);

    // generate random nonce
    let n = randombytes::randombytes(12);
    let nonce = GenericArray::from_slice(n.as_ref()); // 96-bits; unique per message

    // encrypt the contents of the file
    let ciphertext = cipher.encrypt(nonce, contents.as_ref()).unwrap();

    // now we can write the encrypted contents in a new file

    // cipher the name for more security?
    let enc_name = String::from(filename) + ".locked";

    let enc_path = Path::new(&enc_name);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&enc_path) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(ciphertext.as_ref()) {
        Err(why) => panic!("couldn't write to: {}", why),
        Ok(_) => (),
    }

    // return the key underwhich the file was encrypted and the nounce used
    (enc_name, key.as_ref().to_vec(), n)
}

pub fn decrypt_file(filename: &str, key: &[u8], nonce: &[u8]) {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't read file {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let contents: String = reader.lines().map(|l| l.unwrap()).collect();

    let key = GenericArray::clone_from_slice(key);
    let nonce = GenericArray::clone_from_slice(nonce);

    let cipher = Aes256Gcm::new(&key);

    let decrypted = cipher.decrypt(&nonce, contents.as_ref()).unwrap();
    let plaintext = str::from_utf8(decrypted.as_slice()).unwrap();

    let enc_name = String::from(filename) + ".unlocked";
    let enc_path = Path::new(&enc_name);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&enc_path) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(plaintext.as_ref()) {
        Err(why) => panic!("couldn't write to: {}", why),
        Ok(_) => (),
    }
}

// let key = randombytes::randombytes(123);

// let data_part_1 = b"some data";
// let data_part_2 = b"some other data";
// let mut state = auth::State::init(&key);
// state.update(data_part_1);
// state.update(data_part_2);
// let tag1 = state.finalize();

// println!("{:?}", tag1.as_ref());

// let (pk, sk) = box_::gen_keypair();

// println!("Public Key: {}", base64::encode(pk));
// println!("Secret Key: {}", base64::encode(sk));
// // let key2 = base64::encode(pk);
// // let og = base64::decode(key2).unwrap();
// // let tt: &[u8] = og.as_ref();

// // let test = PublicKey::from_slice(tt).unwrap();
// // println!("{:?}", test);
// // println!("{:?}", pk);

// let passwd = b"1L0v3B0b$";
// let salt = pwhash::gen_salt();
// println!("Salt: {}", base64::encode(salt));
// // let enc = base64::encode(salt);
// // // println!("{:?}", enc);
// // let dec = base64::decode(enc).unwrap();
// // let dec_salt = Salt::from_slice(dec.as_ref()).unwrap();
// // println!("{:?}", dec_salt);

// let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
// {
//     let secretbox::Key(ref mut kb) = k;
//     pwhash::derive_key(
//         kb,
//         passwd,
//         &salt,
//         pwhash::OPSLIMIT_INTERACTIVE,
//         pwhash::MEMLIMIT_INTERACTIVE,
//     )
//     .unwrap();

//     println!("Shared secret: {}", base64::encode(kb));
//     // let key = secretbox::Key::from_slice(kb).unwrap();
//     // println!("{:?}", kb);
//     // let ff = base64::encode(key);
//     // println!("{:?}", ff);
//     // println!("{:?}", base64::encode(kb));

//     // let decoded_key = base64::decode(ff).unwrap();
//     // let key = secretbox::Key::from_slice(decoded_key.as_ref()).unwrap();
//     // println!("{:?}", decoded_key);
// }
