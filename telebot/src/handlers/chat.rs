use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use teloxide::{prelude::Message, requests::Requester};

use crate::{commands::{help, start, stop, _404}, constants::BOT, handlers::message::command_handler, helpers::commands::Command};

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
pub async fn private_chat_handler(message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = Command::from_str(&cmd.unwrap().split(' ').collect::<Vec<&str>>()[0]);
    
    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;

    match cmd.unwrap() {
        Command::Start => {
            _404::handle_404(chat_id, None).await;
        },
        Command::Stop => {
            _404::handle_404(chat_id, None).await;
        },
        Command::Help => {
            help::handle_help(chat_id, None).await;
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
pub async fn public_chat_supergroup_handler(pool: Pool<ConnectionManager<PgConnection>>, message: Message) {
    let cmd = command_handler(message.clone());
    if cmd.is_none() {
        return
    }

    let cmd = cmd.unwrap();

    let cmd: Vec<&str> = cmd.split(|c| {
        c == ' ' || c == '@'
    }).collect::<Vec<&str>>();
    
    if cmd.len() <= 1 {
        return;
    }

    let bot_user = BOT.get_me().await.unwrap().username.clone().unwrap();

    if cmd[1].ne(bot_user.as_str()) {
        return;
    }

    let cmd = Command::from_str(cmd[0]);
    
    if cmd.is_none() {
        return
    }

    let chat_id = message.chat.id;
    let thread_id = message.thread_id;

    match cmd.unwrap() {
        Command::Start => {
            start::handle_start(pool, chat_id, thread_id).await;
        },
        Command::Stop => {
            stop::handle_stop(pool, chat_id, thread_id).await;
        },
        Command::Help => {
            help::handle_help(chat_id, thread_id).await;
        }
    }
}