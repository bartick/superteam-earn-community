// Including the modules from the bot
mod earn;

// Files to be included
mod environments;

// Importing the modules as required
use earn::watch::watch;
use environments::load_env;
use database::connection::get_connection_pool;
use dc_commands::connection::connect;

/**
 * This function is the entry point of the bot
 */
#[tokio::main]
async fn main() {
    // load the environment variables
    load_env();

    let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment");

    // Connect to the database
    let pool = get_connection_pool(database_url);

    // check if the connection is successful
    match pool.get() {
        Ok(_) => println!("Connection to the database successful"),
        Err(e) => {
            println!("Error connecting to the database: {}", e);
            return;
        }
    }

    // Run the watch function
    watch(pool.clone()).await;

    connect(pool).await;
}
