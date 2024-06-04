use diesel::{pg::PgConnection, r2d2::{Pool, ConnectionManager}};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder().build(manager).expect("Failed to create pool")
}