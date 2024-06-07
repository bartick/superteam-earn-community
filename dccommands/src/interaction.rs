use twilight_http::Client;
use twilight_model::{application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate};
use crate::handler::get_commands;

pub async fn handle_interaction(client: Client, context: Box<InteractionCreate>) {

    let context_data = context.data.clone();

    match context_data {
        Some(data) => {
            match data {
                InteractionData::ApplicationCommand(data) => {
                    get_commands(client, context, data).await;
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