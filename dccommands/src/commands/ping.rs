use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

pub(crate) fn register(name: &str) -> Command {
    CommandBuilder::new(name, "Pong!", CommandType::ChatInput)
        .option(
            StringBuilder::new("message", "The message to echo back")
                .required(false)
                .build(),
        )
        .option(
            BooleanBuilder::new("uppercase", "Whether to return the message in uppercase")
                .required(false)
                .build(),
        )
        .build()
}