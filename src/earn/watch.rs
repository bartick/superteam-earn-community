use chrono::{Days, Utc};
use crate::earn::constants::EarnUrl;

pub async fn watch() {
    let now = Utc::now() - Days::new(5);
    let now = now.format("%Y-%m-%dT00:00:00.000Z").to_string();

    // Get the project and bounty URLs
    let project_url = EarnUrl::project_url(&now);
    let bounty_url = EarnUrl::bounty_url(&now);

    // Print the URLs
    println!("Project URL: {}", project_url);
    println!("Bounty URL: {}", bounty_url);
}