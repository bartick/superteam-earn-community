use teloxide::prelude::{Bot, Message};

use crate::{commands::{help::handle_help, _404::handle_404}, handlers::message::command_handler, helpers::commands::Command};

/**
 * Private chat handler function
 * 
 * @param bot: Bot
 * @param message: Message
 * 
 * @return ()
 * 
 * @example private_chat_handler(bot, message).await
 * 
 * @description This function is used to handle private chat messages from the user
 */
pub async fn private_chat_handler(bot:Bot, message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = Command::from_str(&cmd.unwrap());
    
    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;

    match cmd.unwrap() {
        Command::Start => {
            handle_404(bot, chat_id).await;
        },
        Command::Stop => {
            handle_404(bot, chat_id).await;
        },
        Command::Help => {
            handle_help(bot, chat_id).await;
        }
    }
}

/**
 * Public chat channel handler function
 * 
 * @param bot: Bot
 * @param message: Message
 * 
 * @return ()
 * 
 * @example public_chat_channel_handler(bot, message).await
 * 
 * @description This function is used to handle public chat messages from the channel
 */
pub async fn public_chat_channel_handler(bot:Bot, message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = Command::from_str(&cmd.unwrap().split(" ").collect::<Vec<&str>>()[0]);

    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;

    match cmd.unwrap() {
        Command::Start => {
            handle_404(bot, chat_id).await;
        },
        Command::Stop => {
            handle_404(bot, chat_id).await;
        },
        Command::Help => {
            handle_help(bot, chat_id).await;
        }
    }
}

/**
 * Public chat group handler function
 * 
 * @param bot: Bot
 * @param message: Message
 * 
 * @return ()
 * 
 * @example public_chat_group_handler(bot, message).await
 * 
 * @description This function is used to handle public chat messages from the group
 */
pub async fn public_chat_group_handler(bot:Bot, message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = Command::from_str(&cmd.unwrap().split(" ").collect::<Vec<&str>>()[0]);
    
    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;

    match cmd.unwrap() {
        Command::Start => {
            handle_404(bot, chat_id).await;
        },
        Command::Stop => {
            handle_404(bot, chat_id).await;
        },
        Command::Help => {
            handle_help(bot, chat_id).await;
        }
    }
}

/**
 * Public chat supergroup handler function
 * 
 * @param bot: Bot
 * @param message: Message
 * 
 * @return ()
 * 
 * @example public_chat_supergroup_handler(bot, message).await
 * 
 * @description This function is used to handle public chat messages from the supergroup
 */
pub async fn public_chat_supergroup_handler(bot:Bot, message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = Command::from_str(&cmd.unwrap().split(" ").collect::<Vec<&str>>()[0]);
    
    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;

    match cmd.unwrap() {
        Command::Start => {
            handle_404(bot, chat_id).await;
        },
        Command::Stop => {
            handle_404(bot, chat_id).await;
        },
        Command::Help => {
            handle_help(bot, chat_id).await;
        }
    }
}