use database::{models::{posts::Post, telegram::Telegram}, schema::telegram};
use diesel::{r2d2::{ConnectionManager, Pool}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use teloxide::{payloads::SendMessageSetters, requests::Requester, types::ChatId};
use crate::constants::BOT;

pub async fn report(pool: Pool<ConnectionManager<PgConnection>>, bounties: Vec<Post>, projects: Vec<Post>) {
    let connection = &mut pool.get().expect("Failed to get connection");

    let settings: Vec<Telegram> = telegram::dsl::telegram.select(telegram::all_columns).filter(telegram::can_send_messages.eq(true)).load::<Telegram>(connection).expect("Failed to get Telegram settings");

    let messages = create_messages(bounties, projects);

    let mut total_disabled_chat: Vec<i64> = Vec::new();

    for message in messages {
        for setting_chunk in settings.chunks(30) {
            let mut handles: Vec<tokio::task::JoinHandle<Option<i64>>> = Vec::new();
    
            for setting in setting_chunk.to_vec() {
                let cloned_messages = message.clone();

                if total_disabled_chat.contains(&setting.id) {
                    continue;
                }

                let handle = tokio::spawn(async move {
                    send_report(setting.id, setting.thread_id, cloned_messages).await
                });
    
                handles.push(handle);
            }

            for handle in handles {
                let disable_chat = handle.await.expect("Failed to send message");

                if disable_chat.is_some() {
                    total_disabled_chat.push(disable_chat.unwrap());
                    diesel::update(telegram::dsl::telegram.filter(telegram::id.eq(disable_chat.unwrap())))
                        .set(telegram::can_send_messages.eq(false))
                        .execute(connection)
                        .expect("Failed to update telegram setting");
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

fn create_messages(bounties: Vec<Post>, projects: Vec<Post>) -> Vec<String> {
    let mut messages: Vec<String> = Vec::new();

    // one message can have maximum of 4096 characters
    let mut message = String::new();
    let all_post: Vec<Post> = bounties.iter().chain(projects.iter()).cloned().collect();

    for post in all_post {
        let post_message = format!("{}\nhttps://earn.superteam.fun/listings/{}/{}\n\n", 
            post.title.unwrap_or(String::from("Title Not Found")),
            post._type.unwrap_or(String::from("type")), 
            post.slug.unwrap_or(String::from("slug-not-found"))
        );

        if message.len() + post_message.len() > 4096 {
            messages.push(message.clone());
            message = post_message;
        } else {
            message.push_str(&post_message);
        }
    }

    if !message.is_empty() {
        messages.push(message);
    }

    messages
}

async fn send_report(chat_id: i64, thread_id: Option<i32>, text: String) -> Option<i64> {
    let chat = ChatId(chat_id);
    let mut message = BOT.send_message(chat, text);

    if thread_id.is_some() {
        let thread_id = thread_id.unwrap();
        message = message.message_thread_id(thread_id);
    }

    let message = message.await;

    if message.is_err() {
        println!("Failed to send message to chat: {:?}", message.err());
        return Some(chat_id)
    }
    None
}