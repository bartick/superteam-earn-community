use database::{models::discord::{DiscordSettings, NewDiscordSettings}, schema::discord_settings};
use diesel::{insert_into, pg::PgConnection, r2d2::{ConnectionManager, Pool}, RunQueryDsl, query_dsl::QueryDsl};

use super::get::get_settings;

pub fn set_settings(pool: Pool<ConnectionManager<PgConnection>> , guild_id: i64, bounty_id: i64, project_id: i64) {

    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let new_settings = NewDiscordSettings {
        id: guild_id,
        bounty_id: Some(bounty_id),
        project_id: Some(project_id)
    };

    // available_value is the result of the query
    let available_value = get_settings(pool, guild_id);

    match available_value {
        Some(settings) => {
            let new_settings_as_settings: DiscordSettings = new_settings.into(); // Convert new_settings to type DiscordSettings

            if settings.ne(&new_settings_as_settings) {
                println!("Updating settings with for guild with id {:?}", settings.id);
                diesel::update(discord_settings::dsl::discord_settings.find(settings.id))
                    .set(&new_settings_as_settings)
                    .execute(connection)
                    .expect("Error updating data");
            }

        },
        None => {
            println!("New settings created for guild with id {:?}", new_settings.id);
            insert_into(discord_settings::dsl::discord_settings)
                .values(&new_settings)
                .execute(connection)
                .expect("Error inserting data");
        }
    }
}