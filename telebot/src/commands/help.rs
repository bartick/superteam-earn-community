use teloxide::{payloads::SendMessageSetters, requests::Requester, types::ChatId};

use crate::constants::BOT;

/**
 * Handle help command
 * 
 * @param bot: Bot
 * @param chat_id: ChatId
 * 
 * @return ()
 * 
 * @example handle_help(bot, chat_id).await
 * 
 * @description This function is used to handle the help command
 */
pub async fn handle_help(chat_id: ChatId, thread_id: Option<i32>) {
    let help_message = "Welcome to the help menu 🤖

/start - Start the bot sending you notifications
/stop - Stop the bot sending you notifications
/help - Display this help message";

    let mut message_to_send = BOT.send_message(chat_id, help_message);

    if thread_id.is_some() {
        message_to_send = message_to_send.message_thread_id(thread_id.unwrap());
    }

    message_to_send.await.expect("Failed to send message");
}