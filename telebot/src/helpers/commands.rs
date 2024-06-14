use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    Start,
    Stop,
    Help,
}

impl Command {
    /**
     * Parse command from string
     * 
     * @param s: &str
     * 
     * @return Option<Self>
     * 
     * @example Command::from_str("/start")
     * 
     * @description This function is used to parse a command from a string
     */
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "/start" => Some(Self::Start),
            "/stop" => Some(Self::Stop),
            "/help" => Some(Self::Help),
            _ => None
        }
    }
}