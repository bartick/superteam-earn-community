use twilight_gateway::{Intents, Shard, ShardId};
use crate::events::handle_event;
use crate::utils::constants::TOKEN;

/**
 * 
 * This function is used to connect to the discord gateway
 * and listen for events
 * 
 */
pub async fn connect() {
    let intents = Intents::GUILD_INTEGRATIONS;
    // Initialize the first and only shard in use by a bot.
    let mut shard = Shard::new(ShardId::ONE, TOKEN.to_string(), intents);

    println!("discord started shard");

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

        tokio::spawn(async move {
            handle_event(event).await
        });
    }
}