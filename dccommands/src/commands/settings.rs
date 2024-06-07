use twilight_model::{application::command::{Command, CommandType}, guild::Permissions};
use twilight_util::builder::command::{ChannelBuilder, CommandBuilder, SubCommandBuilder};

pub(crate) fn register(name: &str) -> Command {
    CommandBuilder::new(name, "Settings of the discord bot", CommandType::ChatInput)
        .default_member_permissions(Permissions::ADMINISTRATOR | Permissions::MANAGE_GUILD)
        .option(
            SubCommandBuilder::new("sll", "Set same channel for bounties and projects")
                .option(
                    ChannelBuilder::new("channel", "Set the channel for bounties and projects")
                        .required(true)
                        .build()
                )
                .build()
        )
        .option(
            SubCommandBuilder::new("set", "Set seperate channel for bounties and projects")
                .option(
                    ChannelBuilder::new("bounty", "Set the bounties channel")
                        .required(true)
                        .build()
                )
                .option(
                    ChannelBuilder::new("project", "Set the projects channel")
                        .required(true)
                        .build()
                )
                .build()
        )
        .option(
            SubCommandBuilder::new("show", "Shows the current settings")
                .build()
        )
        .build()
}