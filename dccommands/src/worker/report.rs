use diesel::{pg::PgConnection, query_dsl::QueryDsl, r2d2::{ConnectionManager, Pool}, ExpressionMethods, RunQueryDsl};
use twilight_http::Client;
use twilight_model::id::Id;
use database::{models::{discord::DiscordSettings, posts::Post}, schema::discord_settings};

use crate::utils::get::get_client;

pub async fn report(pool: Pool<ConnectionManager<PgConnection>>, post: Post) {
    let servers = get_servers().await;

    let connection = &mut pool.get().expect("Failed to get connection");

    // get DiscordSettings from database
    let settings: Vec<DiscordSettings> = discord_settings::dsl::discord_settings.filter(discord_settings::id.eq_any(servers)).load::<DiscordSettings>(connection).expect("Failed to get DiscordSettings");

    for setting in settings {
        let cloned_post = post.clone();
        tokio::spawn(async move {
            send_report(setting, cloned_post).await;
        });
    }
}

async fn get_servers() -> Vec<i64> {
    let client = get_client();
    let guilds = client.current_user_guilds()
        .await
        .expect("Failed to get guilds")
        .models()
        .await
        .expect("Failed to get guilds");

    let mut guild_ids = Vec::new();

    for guild in guilds {
        guild_ids.push(guild.id.get() as i64);
    }

    guild_ids
}

async fn send_report(setting: DiscordSettings, post: Post){
    let channel: u64;
    if post._type == Some(String::from("bounty")) {
        channel = setting.bounty_id.expect("Not found bounty channel") as u64;
    } else if post._type == Some(String::from("project")) {
        channel = setting.project_id.expect("Not found project channel") as u64;
    } else {
        return;
    }
    let client = get_client();
    send_message(&client, channel, format!("**{}**\nhttps://earn.superteam.fun/listings/{}/{}", post.title.unwrap_or(String::from("Title Not Found")), post._type.unwrap(),post.slug.unwrap_or(String::from("")))).await;
}

async fn send_message(client: &Client, channel_id: u64, message: String) {
    client.create_message(Id::new(channel_id))
        .content(message.as_str())
        .expect(format!("Failed to send message to channel: {}", channel_id).as_str())
        .await
        .expect(format!("Failed to send message to channel: {}", channel_id).as_str());
}