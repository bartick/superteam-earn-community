use std::env;
use once_cell::sync::Lazy;

pub(crate) static TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment")
});