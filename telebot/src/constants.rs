use std::env;
use once_cell::sync::Lazy;

pub(crate) static TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELOXIDE_TOKEN").expect("Expected TELOXIDE_TOKEN in the environment")
});