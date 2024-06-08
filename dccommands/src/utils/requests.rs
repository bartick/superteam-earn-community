use twilight_http::Client;
use twilight_model::{channel::message::MessageFlags, gateway::payload::incoming::InteractionCreate, http::interaction::{InteractionResponse, InteractionResponseType}};
use twilight_util::builder::InteractionResponseDataBuilder;

pub async fn set_message_loading(client: &Client,  interaction: Box<InteractionCreate>) {
    client
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &InteractionResponse {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::default()
                    .flags(MessageFlags::EPHEMERAL)
                    .build()
            ),
        })
        .await
        .expect("Failed to send message");
}