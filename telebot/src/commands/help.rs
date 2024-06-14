use teloxide::{prelude::Bot, requests::Requester, types::ChatId};

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
pub async fn handle_help(bot: Bot, chat_id: ChatId) {
    let help_message = "Welcome to the help menu 🤖

/start - Start the bot sending you notifications
/stop - Stop the bot sending you notifications
/help - Display this help message";

    bot.send_message(chat_id, help_message).await.expect("Failed to send message");
}