use std::env;
use once_cell::sync::Lazy;
use teloxide::{adaptors::cache_me::CacheMe, prelude::Bot, requests::RequesterExt};

pub(crate) static TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELOXIDE_TOKEN").expect("Expected TELOXIDE_TOKEN in the environment")
});

pub(crate) static BOT: Lazy<CacheMe<Bot>> = Lazy::new(|| {
    Bot::new(TOKEN.to_string())
        .cache_me()
});