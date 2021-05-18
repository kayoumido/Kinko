use sodiumoxide::crypto::auth::{self, Tag};
use sodiumoxide::crypto::pwhash::{self, Salt};
use sodiumoxide::crypto::secretbox;

///
fn compute_shared_secret(passwd: &str, salt: &str) -> Vec<u8> {
    // let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    let salt = Salt::from_slice(salt.as_bytes()).unwrap();

    let mut key = [0u8; secretbox::KEYBYTES];
    // argon2id13::derive_key(&mut key, password.as_bytes(), &salt,
    //     argon2id13::OPSLIMIT_INTERACTIVE,
    //     argon2id13::MEMLIMIT_INTERACTIVE).unwrap();
    // let secretbox::Key(ref mut kb) = k;
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
fn sign_session_token(token: &[u8], secret: &[u8]) -> Vec<u8> {
    let mut state = auth::State::init(secret);
    state.update(token);
    state.finalize().as_ref().to_vec()
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
