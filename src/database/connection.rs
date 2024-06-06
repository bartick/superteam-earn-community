use std::env;
use once_cell::sync::Lazy;
use diesel::{pg::PgConnection, r2d2::{Pool, ConnectionManager}};

/**
 * This is the database URL
 */
pub(crate) static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment")
});

/**
 * This function returns a connection pool to the database
 */
pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL.to_string());
    Pool::builder().build(manager).expect("Failed to create pool")
}