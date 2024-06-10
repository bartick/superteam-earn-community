use twilight_gateway::{ConfigBuilder, Intents, Shard, ShardId};
use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};
use twilight_model::gateway::{payload::outgoing::update_presence::UpdatePresencePayload, presence::{Activity, ActivityType, Status}};
use crate::events::handle_event;
use crate::utils::constants::TOKEN;

/**
 * 
 * This function is used to connect to the discord gateway
 * and listen for events
 * 
 */
pub async fn connect(pool: Pool<ConnectionManager<PgConnection>>) {
    let intents = Intents::GUILD_INTEGRATIONS;

    let config = ConfigBuilder::new(TOKEN.to_string(), intents)
        .presence(UpdatePresencePayload::new(vec![Activity{
            name: String::from("superteam earn"),
            kind: ActivityType::Watching,
            application_id: None,
            assets: None,
            created_at: None,
            details: None,
            flags: None,
            instance: None,
            party: None,
            secrets: None,
            state: None,
            timestamps: None,
            url: None,
            buttons: vec![],
            emoji: None,
            id: None,
        }], false, Some(0), Status::Online).unwrap())
        .build();

    println!("discord started shard");

    let mut shard = Shard::with_config(ShardId::ONE, config);

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                eprintln!("An error occurred: {:?}", source);

                // If the error is fatal, as may be the case for invalid
                // authentication or intents, then break out of the loop to
                // avoid constantly attempting to reconnect.
                if source.is_fatal() {
                    break;
                }

                continue;
            },
        };

        // You'd normally want to spawn a new tokio task for each event and
        // handle the event there to not block the shard.

        let pass_pool = pool.clone();

        tokio::spawn(async move {
            handle_event(event, pass_pool).await
        });
    }
}