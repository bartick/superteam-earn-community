use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn connect() -> PgConnection  {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&url).unwrap_or_else(|e| {
        panic!("Error connecting to the database: {}", e);
    });
    println!("Connection to the database established!");
    connection
}