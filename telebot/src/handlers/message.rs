use teloxide::{prelude::Message, types::{MediaKind, MessageEntityKind, MessageKind}};


pub fn command_handler(message: Message) -> Option<String> {
    let message_kind = message.kind;

    match message_kind {
        MessageKind::Common(text) => {
            let media_kind = text.media_kind;

            match media_kind {
                MediaKind::Text(msg_cmd) => {

                    println!("cmd: {:?}", msg_cmd.text);

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