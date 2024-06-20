use std::time::Instant;

use twilight_http::Client;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::utils::requests::set_message_loading;

pub(crate) async fn execute(client: Client, interaction: Box<InteractionCreate>) {
    let start = Instant::now();

    set_message_loading(&client, interaction.clone()).await;

    let duration = start.elapsed();

    let message = format!("🏓 WS: `{}`ms", duration.as_millis());

    client
        .interaction(interaction.application_id)
        .update_response(&interaction.token)
        .content(Some(&message))
        .expect("Failed to send message")
        .await
        .expect("Failed to send message");
}