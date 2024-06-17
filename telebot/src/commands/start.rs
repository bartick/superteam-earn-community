use teloxide::{payloads::SendMessageSetters, requests::Requester, types::ChatId};
use diesel::{pg::PgConnection, query_dsl::QueryDsl, r2d2::{ConnectionManager, Pool}, ExpressionMethods, RunQueryDsl};
use database::{models::telegram::{NewTelegram, Telegram}, schema::telegram};

use crate::constants::BOT;


/**
 * This function is used to send the start messaging to the user.
 * 
 * @param chat_id: ChatId - The chat id of the user.
 * @param thread_id: Option<i32> - The thread id of the message.
 * 
 * @returns: () - Returns nothing.
 * 
 * @usage: handle_start(chat_id, thread_id).await;
 * 
 * @logic: This function sends the start message to the user. And start sending the earning opportunities to the user.
 */
pub async fn handle_start(pool: Pool<ConnectionManager<PgConnection>>, chat_id: ChatId, thread_id: Option<i32>) {

    let start_message = "Hello there! I am your bounty bot. And from now on, I will be sending all the earning opportunities here.";

    let new_telegram = NewTelegram {
        id: chat_id.0,
        thread_id,
        can_send_messages: true
    };

    let connection = &mut pool.get().expect("Failed to get connection");

    let value_adready_exists: Option<Telegram> = telegram::dsl::telegram.filter(telegram::id.eq(chat_id.0)).first::<Telegram>(connection).ok();

    match value_adready_exists {
        Some(telegram_settings) => {
            let new_telegram_settings: Telegram = new_telegram.into();

            if telegram_settings.ne(&new_telegram_settings) {
                diesel::update(telegram::dsl::telegram.find(telegram_settings.id))
                    .set(&new_telegram_settings)
                    .execute(connection)
                    .expect("Failed to update data");
            }
        },
        None => {
            diesel::insert_into(telegram::table)
                .values(&new_telegram)
                .execute(connection)
                .expect("Failed to insert data");
        }
        
    }

    let mut message_to_send = BOT.send_message(chat_id, start_message);

    if thread_id.is_some() {
        message_to_send = message_to_send.message_thread_id(thread_id.unwrap());
    }

    message_to_send.await.expect("Failed to send message");
}