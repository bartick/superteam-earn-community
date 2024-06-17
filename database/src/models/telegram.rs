use diesel::prelude::{Insertable, Queryable, AsChangeset};
use serde::Serialize;

use crate::schema::telegram;

#[derive(Debug, Insertable, AsChangeset, Serialize)]
#[diesel(table_name = telegram)]
pub struct NewTelegram {
    pub id: i64,
    pub thread_id: Option<i32>,
    pub can_send_messages: bool,
}

#[derive(Debug, Queryable, AsChangeset, Serialize, PartialEq)]
#[diesel(table_name = telegram)]
pub struct Telegram {
    pub id: i64,
    pub thread_id: Option<i32>,
    pub can_send_messages: bool,
}

impl Into<Telegram> for NewTelegram {
    fn into(self) -> Telegram {
        Telegram {
            id: self.id,
            thread_id: self.thread_id,
            can_send_messages: self.can_send_messages,
        }
    }
}