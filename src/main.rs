// Including the modules from the bot
mod earn;
mod database;

// Files to be included
mod environments;

// Importing the modules as required
use earn::watch::watch;
use environments::load_env;
use database::connection::connect;

/**
 * This function is the entry point of the bot
 */
#[tokio::main]
async fn main() {
    // load the environment variables
    load_env();

    // Connect to the database
    let mut connection = connect();

    // Run the watch function
    watch(&mut connection).await;
}
