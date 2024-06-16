use std::fmt::Display;

use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    #[command(description = "Add your chat to send new earning opportunities")]
    Start,
    #[command(description = "Remove your chat from sending new earning opportunities")]
    Stop,
    #[command(description = "Get help on how to use the bot")]
    Help,
}

impl Display for Command {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }

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

    /**
     * Get the command as a string
     * 
     * @param &self
     * 
     * @return String
     * 
     * @example Command::Start.to_string()
     * 
     * @description This function is used to get the command as a string
     */
    fn to_string(&self) -> String {
        match self {
            Self::Start => "/start".to_string(),
            Self::Stop => "/stop".to_string(),
            Self::Help => "/help".to_string()
        }
    }
}