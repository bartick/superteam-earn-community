// Including the modules from the bot
mod earn;

// Files to be included
mod environments;

// Importing the modules as required
use earn::watch::watch;
use environments::load_env;
use database::connection::get_connection_pool;
use dc_commands::connection::connect;
use telebot::run;

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

    let telegram_token = std::env::var("TELOXIDE_TOKEN").expect("Expected TELOXIDE_TOKEN in the environment");

    let discord_pool = pool.clone();
    
    tokio::spawn(async move {
        connect(discord_pool).await;
    });


    tokio::spawn(async move {
        run(telegram_token).await;
    });

    tokio::signal::ctrl_c().await.unwrap();
}
