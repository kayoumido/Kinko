#[macro_use]
extern crate diesel;
extern crate dotenv;

#[path = "client/client.rs"]
mod client;

#[path = "server/server.rs"]
mod server;

mod db;
mod errors;

fn main() {
    db::init();

    println!("Hello, world!");
}
