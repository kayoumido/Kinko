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

    let (_files, secret, session_token) = client::login(username, passwd).unwrap();

    client::upload_file(
        "files/home/passwords.txt",
        username,
        secret.as_ref(),
        session_token.as_ref(),
        pk.as_ref(),
    );

    client::download_file(
        "passwords.txt.locked",
        "AliceXOXO",
        secret.as_ref(),
        session_token.as_ref(),
        sk.as_ref(),
    )
}
