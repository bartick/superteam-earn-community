mod commands;
mod helper;

use helper::CommandRegister;

async fn register_commands(token: &str) {
    use twilight_http::Client;

    // loop CommandRegister


    let command = [
        CommandRegister::Ping.as_command(),
        CommandRegister::Settings.as_command(),
    ];
    let client = Client::new(token.to_string());
    let application_id = client
        .current_user_application()
        .await
        .expect("Failed to get current user application")
        .model()
        .await
        .expect("Failed to get current user application model")
        .id;
    let _ = client.interaction(application_id).set_global_commands(&command).await.expect("Failed to set global commands");
}

#[tokio::main]
async fn main() {
    use dotenv;
    dotenv::from_filename(".env").expect("Failed to load .env file");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");

    register_commands(&token).await;

    println!("Commands registered...")
}