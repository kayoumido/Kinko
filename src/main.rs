#[macro_use]
extern crate diesel;
extern crate base64;
extern crate dotenv;

#[path = "client/client.rs"]
mod client;

#[path = "server/server.rs"]
mod server;

mod db;
mod errors;

use std::str;
/*
    id: 1
    username: AliceXOXO
    password: 1L0v3B0b$

    public_key: BJnRg+bze7YdwWKfKODuwewCqdvCLxEiMbgjGORbwMK1xqRBcWpr8dILxJFlfaqefTBzInRzHtdysSHGAxoj7M8=
    private_key: GEJ3z7uSTOoqredKnm8t6ie/ptATOxhlHqCSGQO0dKE=

    shared_secret: CpP6H2YRfRSStCBwmmB7eAI00rlZCeOKmNXlZ7DyIVY=
    shared_secret_salt: esM6awwBCndMbzph6SvwXggvsE0B38742FU8lokT2P8=
*/

fn main() {
    db::init();
    sodiumoxide::init().unwrap();

    let username = "AliceXOXO";
    let passwd = "1L0v3B0b$";
    let sk = base64::decode("GEJ3z7uSTOoqredKnm8t6ie/ptATOxhlHqCSGQO0dKE=").unwrap();
    let pk = base64::decode(
        "BJnRg+bze7YdwWKfKODuwewCqdvCLxEiMbgjGORbwMK1xqRBcWpr8dILxJFlfaqefTBzInRzHtdysSHGAxoj7M8=",
    )
    .unwrap();

    let (files, secret, session_token) = client::login(username, passwd).unwrap();

    println!("Here are your files:");
    for file in files {
        let encrypted_name = base64::decode_config(file.name, base64::URL_SAFE).unwrap();
        let encrypted_key = base64::decode(file.symmetric_key).unwrap();
        let nonce = base64::decode(file.name_nonce).unwrap();
        let key = client::crypto::decrypt_key(encrypted_key.as_ref(), sk.as_ref());
        let name = client::crypto::_decrypt(encrypted_name.as_ref(), key.as_ref(), nonce.as_ref());
        let name = str::from_utf8(name.as_ref()).unwrap();

        println!("- {}", name);
    }

    println!("Uploading a new file");
    if let Err(why) = client::upload_file(
        "files/home/liip.txt",
        username,
        secret.as_ref(),
        session_token.as_ref(),
        pk.as_ref(),
    ) {
        println!("{}", why);
    } else {
        println!("Upload successful!");
    }

    println!("Download a file");
    if let Err(why) = client::download_file(
        "Fo_ZakTJWBWP9GUgVnoVaqWOU4EIdDUoAThk_R8=",
        "AliceXOXO",
        secret.as_ref(),
        session_token.as_ref(),
        sk.as_ref(),
    ) {
        println!("{}", why);
    } else {
        println!("Download successful! Check your folders");
    }
}
