// Including the modules from the bot
mod earn;

// Files to be included
mod environments;

// Importing the modules as required
use earn::watch::watch;

/**
 * This function is the entry point of the bot
 */
#[tokio::main]
async fn main() {
    // Run the watch function
    environments::load_env();
    watch().await;
}
