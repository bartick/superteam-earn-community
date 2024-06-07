use twilight_http::Client;
use twilight_model::{application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate};
use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};
use crate::{helper::CommandRegister, handler::{ping, settings}};

pub async fn handle_interaction(client: Client, context: Box<InteractionCreate>, pool: Pool<ConnectionManager<PgConnection>>) {

    let context_data = context.data.clone();

    match context_data {
        Some(data) => {
            match data {
                InteractionData::ApplicationCommand(data) => {
                    let handler = CommandRegister::from_str(data.name.as_str());
                    match handler {
                        Some(ctx) => match ctx {
                            CommandRegister::Ping => ping::execute(client, context, data).await,
                            CommandRegister::Settings => settings::execute(client, context, data, pool).await,
                        },
                        _ => {
                            eprintln!("Unknown command: {}", data.name);
                        }
                    }
                },
                InteractionData::MessageComponent(data) => {
                    println!("Message component received: {:?}", data.custom_id);
                },
                _ => {
                    eprintln!("Unhandled interaction data: {:?}", data);
                }
            }
        },
        None => {
            eprintln!("No data found in interaction");
        }
    }
}