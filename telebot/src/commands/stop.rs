use database::schema::telegram;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection, ExpressionMethods, RunQueryDsl, QueryDsl};
use teloxide::{payloads::SendMessageSetters, requests::Requester, types::ChatId};

use crate::constants::BOT;


/**
 * This function is used to send the stop messaging to the user.
 * 
 * @param chat_id: ChatId - The chat id of the user.
 * @param thread_id: Option<i32> - The thread id of the message.
 * 
 * @returns: () - Returns nothing.
 * 
 * @usage: handle_stop(chat_id, thread_id).await;
 * 
 * @logic: This function stop sending the earning opportunities to the user.
 */
pub async fn handle_stop(pool: Pool<ConnectionManager<PgConnection>>, chat_id: ChatId, thread_id: Option<i32>) {

    let start_message = "I am stop going to sending earning opportunities here.";

    diesel::delete(telegram::dsl::telegram.filter(telegram::id.eq(chat_id.0)))
        .execute(&mut pool.get().expect("Failed to get connection")).ok();

    let mut message_to_send = BOT.send_message(chat_id, start_message);

    if thread_id.is_some() {
        message_to_send = message_to_send.message_thread_id(thread_id.unwrap());
    }

    message_to_send.await.expect("Failed to send message");
}