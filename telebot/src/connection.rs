use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use teloxide::{prelude::Message, requests::Requester, types::{ChatKind, PublicChatKind}};

use crate::{constants::BOT, handlers::chat::{private_chat_handler, public_chat_supergroup_handler}};

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
pub async fn run(pool: Pool<ConnectionManager<PgConnection>>) {

    println!("Running telegram bot...");

    let me = BOT.get_me().await.unwrap();

    println!("Logged in as: {}", me.username.clone().expect("Unable to find user"));

    teloxide::repl(BOT.clone(), move |message: Message| {
        let pool = pool.clone();

        async move {
            let channel_type = message.clone().chat.kind;

            match channel_type {
                ChatKind::Private(_) => {
                    private_chat_handler(message).await;
                },
                ChatKind::Public(ctx) => {
                    let public_type = ctx.kind;

                    // Check if the user is admin or not
                    let is_admin = BOT.get_chat_administrators(message.chat.id).await.unwrap().iter().any(|admin| admin.user.id == message.from().unwrap().id);

                    if !is_admin {
                        return Ok(());
                    }

                    match public_type {
                        PublicChatKind::Supergroup(_) => {
                            public_chat_supergroup_handler(pool, message).await;
                        },
                        _ => {}
                    }
                }
            }
            Ok(())
        }
    })
    .await;
}