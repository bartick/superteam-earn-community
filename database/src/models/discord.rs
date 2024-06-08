use diesel::prelude::{Insertable, Queryable, AsChangeset};
use serde::Serialize;

use crate::schema::discord_settings;

#[derive(Debug, Insertable, AsChangeset, Serialize)]
#[diesel(table_name = discord_settings)]
pub struct NewDiscordSettings {
    pub id: i64,
    pub bounty_id: Option<i64>,
    pub project_id: Option<i64>
}

#[derive(Debug, Queryable, AsChangeset, Serialize, Eq, PartialEq)]
#[diesel(table_name = discord_settings)]
pub struct DiscordSettings {
    pub id: i64,
    pub bounty_id: Option<i64>,
    pub project_id: Option<i64>
}

impl Into<DiscordSettings> for NewDiscordSettings {
    fn into(self) -> DiscordSettings {
        DiscordSettings {
            id: self.id,
            bounty_id: self.bounty_id,
            project_id: self.project_id
        }
    }
}
