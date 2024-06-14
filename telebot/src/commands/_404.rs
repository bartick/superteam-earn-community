use teloxide::{prelude::Bot, requests::Requester, types::ChatId};

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
pub async fn handle_404(bot: Bot, chat_id: ChatId) {
    let help_message = "This function does not exist for this channel type 🤖";
    bot.send_message(chat_id, help_message).await.expect("Failed to send message");
}