use database::{models::discord::DiscordSettings, schema::discord_settings};
use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}, RunQueryDsl, query_dsl::QueryDsl, ExpressionMethods};
use twilight_http::Client;
use twilight_model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};
use super::constants::TOKEN;

/**
 * This function returns the client
 */
pub fn get_client() -> Client {
    Client::new(TOKEN.to_string())
}

pub fn get_channel_id(channel: CommandDataOption) -> u64 {
    match channel.value {
        CommandOptionValue::Channel(value) => value.get(),
        _ => 0,
    }
}

pub fn get_settings(pool: Pool<ConnectionManager<PgConnection>> , guild_id: i64) -> Option<DiscordSettings> {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let available_value: Option<DiscordSettings> = discord_settings::dsl::discord_settings.filter(discord_settings::id.eq(guild_id)).first::<DiscordSettings>(connection).ok();
    available_value
}