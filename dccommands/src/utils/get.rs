use twilight_http::Client;
use super::constants::TOKEN;

/**
 * This function returns the client
 */
pub fn get_client() -> Client {
    Client::new(TOKEN.to_string())
}