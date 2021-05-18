/*!
 * Database configurations
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

pub mod models;
pub mod repository;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use lazy_static::lazy_static;
use r2d2;
use std::env;

use crate::errors::DBError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let _conn = connection().expect("Failed to get db connection");
}

pub fn connection() -> Result<DBConnection, DBError> {
    POOL.get().map_err(|_| DBError::ConnectionFailed)
}
