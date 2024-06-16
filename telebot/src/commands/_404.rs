use teloxide::{payloads::SendMessageSetters, requests::Requester, types::ChatId};

use crate::constants::BOT;

/**
 * Handle 404 command
 * 
 * @param bot: Bot
 * @param chat_id: ChatId
 * 
 * @return ()
 * 
 * @example handle_404(bot, chat_id).await
 * 
 * @description This function is used to handle the 404 command/handle when a command does not exist for the channel type
 */
pub async fn handle_404(chat_id: ChatId, thread_id: Option<i32>) {
    let help_message = "This function does not exist for this channel type 🤖";
    let mut message_to_send = BOT.send_message(chat_id, help_message);

    if thread_id.is_some() {
        message_to_send = message_to_send.message_thread_id(thread_id.unwrap());
    }

    message_to_send.await.expect("Failed to send message");


}