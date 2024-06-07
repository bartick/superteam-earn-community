use twilight_http::Client;
use crate::discord::connection::TOKEN;

/**
 * This function returns the client
 */
pub fn get_client() -> Client {
    Client::new(TOKEN.to_string())
}