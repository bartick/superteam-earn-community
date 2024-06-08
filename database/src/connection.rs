use diesel::{pg::PgConnection, r2d2::{Pool, ConnectionManager}};

/**
 * This function returns a connection pool to the database
 */
pub fn get_connection_pool(url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url.to_string());
    Pool::builder().build(manager).expect("Failed to create pool")
}