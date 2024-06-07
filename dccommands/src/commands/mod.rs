pub(crate) mod ping;

use crate::helper::CommandRegister;

impl CommandRegister {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            CommandRegister::Ping => "ping",
        }
    }

    pub(crate) fn as_command(&self) -> twilight_model::application::command::Command {
        match self {
            CommandRegister::Ping => ping::register(Self::Ping.as_str()),
        }
    }
}