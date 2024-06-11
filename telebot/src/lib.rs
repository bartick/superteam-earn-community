use teloxide::prelude::*;

pub async fn run(token: String) {
    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        bot.send_message(message.chat.id, "Hello There").await?;
        Ok(())
    })
    .await;
}