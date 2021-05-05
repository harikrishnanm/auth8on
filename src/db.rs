use log::{error, info};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use std::result::Result;
use core::time::Duration;

use std::env;


pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub async fn init() -> Result<PgPool, PoolError> {
    info!("Initializing DB");
    const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const DATABASE_URL: &'static str = env!("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(env!("DATABASE_URL"));

    Pool::builder()
        .max_size(15)
        .min_idle(Some(3))
        .connection_timeout(Duration::new(5, 0))
        .idle_timeout(Some(Duration::new(60, 0)))
        .build(manager)
}
