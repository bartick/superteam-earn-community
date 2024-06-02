// Including the modules from the bot
mod earn;

// Importing the modules as required
use earn::watch::watch;

/**
 * This function is the entry point of the bot
 */
#[tokio::main]
async fn main() {
    // Run the watch function
    watch().await;
}
