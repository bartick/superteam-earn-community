use crate::helper::CommandRegister;

pub(crate) mod ping;
pub(crate) mod settings;

impl CommandRegister {
    pub(crate) fn from_str(name: &str) -> Option<Self> {
        match name {
            "ping" => Some(Self::Ping),
            "settings" => Some(Self::Settings),
            _ => None,
        }
    }
}
