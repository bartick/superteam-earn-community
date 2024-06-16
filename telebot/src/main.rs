#![allow(dead_code)]
mod constants;
mod helpers;

use teloxide::{payloads::SetMyCommandsSetters, prelude::Bot, requests::Requester, types::BotCommandScope, utils::command::BotCommands};
use constants::TOKEN;
use helpers::commands::Command;

#[tokio::main]
async fn main() {
    use dotenv;
    dotenv::from_filename(".env").expect("Failed to load .env file");

    let bot = Bot::new(TOKEN.to_string());

    bot.set_my_commands(<Command as BotCommands>::bot_commands()).scope(BotCommandScope::AllChatAdministrators).await.expect("Failed to set commands");

    println!("Commands registered...")
}