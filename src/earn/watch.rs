use chrono::Utc;
use tokio;
use crate::earn::constants::EarnUrl;

pub async fn watch() {
    // Get the current time in the format required by the API
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    // Get the project and bounty URLs
    let project_url = EarnUrl::project_url(&now);
    let bounty_url = EarnUrl::bounty_url(&now);

    // Print the URLs
    println!("Project URL: {}", project_url);
    println!("Bounty URL: {}", bounty_url);

    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
}