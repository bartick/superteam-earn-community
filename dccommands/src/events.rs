use twilight_gateway::Event;
use twilight_model::{application::interaction::InteractionType, http::interaction::{InteractionResponse, InteractionResponseType}};
use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};

use crate::interaction::handle_interaction;

use crate::utils::get::get_client;

pub async fn handle_event(event: Event, pool: Pool<ConnectionManager<PgConnection>>) {
    let client = get_client();
    match event {
        Event::Ready(_) => {
            println!("Bot is ready");
        },
        Event::InteractionCreate(interaction) => {
            // check interaction type
            match interaction.kind {
                InteractionType::Ping => {
                    println!("Ping received");
                    client.interaction(interaction.application_id)
                        .create_response(
                            interaction.id, 
                            &interaction.token,
                            &InteractionResponse {
                                kind: InteractionResponseType::Pong,
                                data: None,
                            }
                        )
                        .await
                        .unwrap();
                },
                InteractionType::ApplicationCommand => {
                    handle_interaction(client, interaction, pool).await;
                },
                InteractionType::ApplicationCommandAutocomplete => {
                    println!("Command autocomplete received");
                    // handle_interaction(interaction).await;
                },
                InteractionType::MessageComponent => {
                    println!("Message component received");
                    // handle_interaction(interaction).await;
                },
                InteractionType::ModalSubmit => {
                    println!("Modal submit received");
                    // handle_interaction(interaction).await;
                },
                _ => {
                    eprintln!("Unhandled interaction type: {:?}", interaction.kind);
                }
            }
        },
        _ => {},
    }
}