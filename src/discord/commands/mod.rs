mod ping;
use twilight_http::Client;
use twilight_model::{application::interaction::application_command::CommandData, gateway::payload::incoming::InteractionCreate};

pub async fn get_commands(client: Client, interaction: Box<InteractionCreate>, data: Box<CommandData>) -> Option<()>  {
    let name = data.name.as_str();
    match name {
        "ping" => Some(ping::send_message(client, interaction, data).await),
        _ => {
            eprintln!("Unknown command: {}", name);
            None
        }
    }
}