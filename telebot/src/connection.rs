use teloxide::{prelude::*, types::{ChatKind, PublicChatKind}};

use crate::handlers::chat::{private_chat_handler, public_chat_channel_handler, public_chat_group_handler, public_chat_supergroup_handler};

/**
 * Run the bot
 * 
 * @param token: String
 * 
 * @return ()
 * 
 * @example run(token).await
 * 
 * @description This function is used to run the bot
 */
pub async fn run(token: String) {
    let bot = Bot::new(token);

    println!("Running telegram bot...");

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        let channel_type = message.clone().chat.kind;
        match channel_type {
            ChatKind::Private(_) => {
                private_chat_handler(bot, message).await;
            },
            ChatKind::Public(ctx) => {
                let public_type = ctx.kind;
                match public_type {
                    PublicChatKind::Channel(_) => {
                        public_chat_channel_handler(bot, message).await;
                    },
                    PublicChatKind::Group(_) => {
                        public_chat_group_handler(bot, message).await;
                    },
                    PublicChatKind::Supergroup(_) => {
                        public_chat_supergroup_handler(bot, message).await;
                    }
                }
            }
        }
        Ok(())
    })
    .await;
}