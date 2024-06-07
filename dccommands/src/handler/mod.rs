mod ping;
use twilight_http::Client;
use twilight_model::{application::interaction::application_command::CommandData, gateway::payload::incoming::InteractionCreate};
use crate::helper::CommandRegister;

impl CommandRegister {
    pub(crate) fn from_str(name: &str) -> Option<Self> {
        match name {
            "ping" => Some(Self::Ping),
            _ => None,
        }
    }
}

pub async fn get_commands(client: Client, interaction: Box<InteractionCreate>, data: Box<CommandData>) -> Option<()>  {
    let handler = CommandRegister::from_str(data.name.as_str());
    match handler {
        Some(ctx) => match ctx {
            CommandRegister::Ping => Some(ping::execute(client, interaction, data).await),
        },
        _ => {
            eprintln!("Unknown command: {}", data.name);
            None
        }
    }
}