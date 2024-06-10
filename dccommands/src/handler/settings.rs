use std::borrow::Borrow;

use twilight_http::Client;
use twilight_model::{application::interaction::application_command::{CommandData, CommandDataOption, CommandOptionValue}, channel::message::MessageFlags, gateway::payload::incoming::InteractionCreate, http::interaction::{InteractionResponse, InteractionResponseType}};
use twilight_util::builder::{embed::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource}, InteractionResponseDataBuilder};
use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};
use crate::utils::{get::{get_channel_id, get_settings}, set::set_settings, requests::set_message_loading};

fn all_channel(pool: Pool<ConnectionManager<PgConnection>>, guild_id: i64, choise: CommandDataOption) -> [i64; 2] {
    let channel = get_channel_id(choise) as i64;
    set_settings(pool, guild_id, channel, channel);
    [channel, channel]
}

fn set_channel(pool: Pool<ConnectionManager<PgConnection>>, guild_id: i64, choise: Vec<CommandDataOption>) -> [i64; 2] {
    let mut bounty_channel: i64 = 0;
    let mut project_channel: i64 = 0;
    for option in choise {
        if option.name == "bounty" {
            bounty_channel = get_channel_id(option) as i64;
        } else if option.name == "project" {
            project_channel = get_channel_id(option) as i64;
        }
    }

    set_settings(pool, guild_id, bounty_channel, project_channel);

    [bounty_channel, project_channel]
}

fn show_channel(pool: Pool<ConnectionManager<PgConnection>>, guild_id: i64) -> [i64; 2] {

    let settings = get_settings(pool, guild_id);

    if settings.is_some() {
        let settings = settings.unwrap();
        let bounty_channel = settings.bounty_id.unwrap_or(0);
        let project_channel = settings.project_id.unwrap_or(0);
        return [bounty_channel, project_channel];
    } else {
        return [0, 0];
    }
}

pub(crate) async fn execute(client: Client, interaction: Box<InteractionCreate>, data: Box<CommandData>, pool: Pool<ConnectionManager<PgConnection>>) {

    set_message_loading(client.borrow(), interaction.clone()).await;

    let content = String::from("The Settings for the bot are:");

    let subcommand = data.options[0].name.clone();

    let [mut bounty_channel, mut project_channel]: [i64; 2] = [0, 0];

    let user = interaction.author().unwrap();

    let user_avatar = match ImageSource::url(format!("https://cdn.discordapp.com/avatars/{}/{}.png", user.id, user.avatar.unwrap().to_string())) {
        Ok(avatar) => Some(avatar),
        Err(_) => None,
    };

    let footer = EmbedFooterBuilder::new(user.name.clone());
    let footer = match user_avatar {
        Some(avatar) => footer.icon_url(avatar),
        None => footer,
    };

    if interaction.is_dm() {
        let response: InteractionResponse = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseDataBuilder::new()
                .embeds(
                    vec![
                        EmbedBuilder::new()
                            .title("Settings")
                            .description("Nothing to show, this is a DM.")
                            .color(0x00ff00)
                            .footer(footer)
                            .build()
                    ]
                )
                .flags(MessageFlags::EPHEMERAL)
                .build()
            ),
        };
    
        client
            .interaction(interaction.application_id)
            .create_response(interaction.id, &interaction.token, &response)
            .await.unwrap();
        return;
    }

    let guild_id = interaction.guild_id.unwrap().get() as i64;

    if subcommand == "show" {
        [bounty_channel, project_channel] = show_channel(pool, guild_id);
    } else {
        let choise = match data.options[0].value.clone() {
            CommandOptionValue::SubCommand(choise) => choise,
            _ => return,
        };
        if subcommand == "all" {
            [bounty_channel, project_channel] = all_channel(pool, guild_id, choise[0].clone());
        } else if subcommand == "set" {
            [bounty_channel, project_channel] = set_channel(pool, guild_id, choise);
        }
    }

    client
        .interaction(interaction.application_id)
        .update_response(&interaction.token)
        .embeds(Some(
            &vec![
                EmbedBuilder::new()
                    .title("Settings")
                    .description(content)
                    .field(EmbedFieldBuilder::new("Bounty Channel: ", format!("<#{}>", bounty_channel)).inline().build())
                    .field(EmbedFieldBuilder::new("Project Channel: ", format!("<#{}>", project_channel)).inline().build())
                    .footer(footer.build())
                    .color(0x00ff00)
                    .build()
            ]
        ))
        .expect("Failed to create a message")
        .await
        .expect("Failed to send message");
        // .create_response(interaction.id, &interaction.token, &response)
}