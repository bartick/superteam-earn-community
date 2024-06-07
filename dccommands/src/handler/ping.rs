use twilight_http::Client;
use twilight_model::{application::interaction::application_command::{CommandData, CommandOptionValue}, channel::message::MessageFlags, gateway::payload::incoming::InteractionCreate, http::interaction::{InteractionResponse, InteractionResponseType}};
use twilight_util::builder::InteractionResponseDataBuilder;

pub(crate) async fn execute(client: Client, interaction: Box<InteractionCreate>, data: Box<CommandData>) {
    let mut content = String::from("Pong!");

    for options in data.options {
        if options.name == "message" { 
            match options.value {
                CommandOptionValue::String(value) => {
                    content = value;
                },
                _ => {}
            } 
        }
        else if options.name == "uppercase" {
            match options.value.clone() {
                CommandOptionValue::Boolean(value) => {
                    if value {
                        content = content.to_uppercase();
                    }
                },
                _ => {}
            }
        }
        else {}
    }

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