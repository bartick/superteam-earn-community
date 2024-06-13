use teloxide::prelude::{Bot, Message};

use super::message::command_handler;


pub async fn private_chat_handler(_bot:Bot, message: Message) {
    let cmd = command_handler(message);
    println!("Command: {:?}", cmd);
}

pub async fn public_chat_channel_handler(_bot:Bot, message: Message) {
    let cmd = command_handler(message);
    println!("Command: {:?}", cmd);
}

pub async fn public_chat_group_handler(_bot:Bot, message: Message) {
    let cmd = command_handler(message);
    println!("Command: {:?}", cmd);
}

pub async fn public_chat_supergroup_handler(_bot:Bot, message: Message) {
    let cmd = command_handler(message);
    println!("Command: {:?}", cmd);
}