use teloxide::{prelude::Message, types::{MediaKind, MessageEntityKind, MessageKind}};

/**
 * Command handler function for messages
 * 
 * @param message: Message
 * 
 * @return Option<String>
 * 
 * @example command_handler(message)
 * 
 * @description This function is used to handle messages and return the command if it is a command
 */
pub fn command_handler(message: Message) -> Option<String> {
    let message_kind = message.kind;

    match message_kind {
        MessageKind::Common(text) => {
            let media_kind = text.media_kind;

            match media_kind {
                MediaKind::Text(msg_cmd) => {

                    if msg_cmd.entities.is_empty() {
                        return None;
                    }

                    match msg_cmd.entities[0].kind {
                        MessageEntityKind::BotCommand => Some(msg_cmd.text),
                        _ => None
                    }
                },
                _ => None
            }
        },
        _ => None
    }
}