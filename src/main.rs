// Including the modules from the bot
mod earn;
mod database;

// Files to be included
mod environments;

// Importing the modules as required
use earn::watch::watch;
use environments::load_env;
use database::connection::get_connection_pool;

/**
 * This function is the entry point of the bot
 */
#[tokio::main]
async fn main() {
    // load the environment variables
    load_env();

    // Connect to the database
    let pool = get_connection_pool();

    // check if the connection is successful
    match pool.get() {
        Ok(_) => println!("Connection to the database successful"),
        Err(e) => {
            println!("Error connecting to the database: {}", e);
            return;
        }
    }

    // Run the watch function
    watch(pool).await;
}
