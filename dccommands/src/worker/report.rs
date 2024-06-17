use diesel::{pg::PgConnection, query_dsl::QueryDsl, r2d2::{ConnectionManager, Pool}, ExpressionMethods, RunQueryDsl};
use twilight_model::id::Id;
use database::{models::{discord::DiscordSettings, posts::Post}, schema::discord_settings};

use crate::utils::get::get_client;

pub async fn report(pool: Pool<ConnectionManager<PgConnection>>, bounties: Vec<Post>, projects: Vec<Post>) {
    let servers = get_servers().await;

    let connection = &mut pool.get().expect("Failed to get connection");

    // get DiscordSettings from database
    let settings: Vec<DiscordSettings> = discord_settings::dsl::discord_settings.filter(discord_settings::id.eq_any(servers)).load::<DiscordSettings>(connection).expect("Failed to get DiscordSettings");

    for setting in settings {
        let cloned_bounties = bounties.clone();
        let cloned_projects = projects.clone();
        tokio::spawn(async move {
            send_report(setting, cloned_bounties, cloned_projects).await;
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

async fn send_report(setting: DiscordSettings, bounties: Vec<Post>, projects: Vec<Post>){
    if setting.bounty_id == setting.project_id {
        let posts: Vec<Post> = bounties.iter().chain(projects.iter()).cloned().collect();
        let channel = setting.bounty_id.unwrap() as u64;
        tokio::spawn(async move {
            send_message(channel, posts).await;
        });
    } else {
        let bounty_channel = setting.bounty_id.unwrap() as u64;
        let project_channel = setting.project_id.unwrap() as u64;
        tokio::spawn(async move {
            send_message(bounty_channel, bounties).await;
        });

        tokio::spawn(async move {
            send_message(project_channel, projects).await;
        });
    }
}

async fn send_message(channel_id: u64, posts: Vec<Post>) {
    let mut _response_number = 0;

    // send 5 tokio spawn response messages per second
    for post_chunk in posts.chunks(5) {

        let mut handles = Vec::new();

        for post in post_chunk.to_vec() {
            let handle = tokio::spawn(async move {
                let client = get_client();
                client.create_message(Id::new(channel_id))
                    .content(format!("**{}**\nhttps://earn.superteam.fun/listings/{}/{}", post.title.unwrap_or(String::from("Title Not Found")), post._type.unwrap(), post.slug.unwrap_or(String::from(""))).as_str())
                    .expect(format!("Failed to send message to channel: {}", channel_id).as_str())
                    .await
                    .expect(format!("Failed to send message to channel: {}", channel_id).as_str());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}