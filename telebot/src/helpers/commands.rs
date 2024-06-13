use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", parse_with = "split")]
enum Command {
    Help,
    Start
}