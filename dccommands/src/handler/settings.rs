use twilight_http::Client;
use twilight_model::{application::interaction::application_command::CommandData, channel::message::MessageFlags, gateway::payload::incoming::InteractionCreate, http::interaction::{InteractionResponse, InteractionResponseType}};
use twilight_util::builder::InteractionResponseDataBuilder;
use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};

pub(crate) async fn execute(client: Client, interaction: Box<InteractionCreate>, _data: Box<CommandData>, _pool: Pool<ConnectionManager<PgConnection>>) {
    let content = String::from("Settings Yet to be implemented");

    let response: InteractionResponse = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new()
            .content(content)
            .flags(MessageFlags::EPHEMERAL)
            .build()
        ),
    };

    client
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await.unwrap();
}