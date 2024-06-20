use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

pub(crate) fn register(name: &str) -> Command {
    CommandBuilder::new(name, "Show Bot Latency 🏓", CommandType::ChatInput)
        .build()
}