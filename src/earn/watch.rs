use chrono::{Days, Utc, NaiveDateTime};
use uuid::Uuid;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use crate::{database::{models::posts::NewPost, schema::posts}, earn::constants::EarnUrl};

pub async fn fetch_data(url: &str) -> serde_json::Value {
    // Fetch the data from the URL
    let response = reqwest::get(url).await.unwrap();

    // Get the body of the response as json
    let body = response.text().await.unwrap();
    let body = serde_json::from_str::<serde_json::Value>(&body).unwrap();

    // Return the value
    body
}

pub async fn add_data_to_database(connection: &mut PgConnection, data: &serde_json::Value) -> bool {
    
    let new_post = NewPost {
        id: Uuid::parse_str(data.get("id").unwrap().as_str().unwrap()).unwrap(),
        title: Some(data.get("title").unwrap_or(&serde_json::Value::Null).to_string()),
        slug: Some(data.get("slug").unwrap().to_string()),
        deadline: Some(NaiveDateTime::parse_from_str(data.get("deadline").unwrap().as_str().unwrap(), "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap()),
        token: Some(data.get("token").unwrap().to_string()),
        rewardamount: match data.get("rewardAmount") {
            Some(reward) => Some(reward.as_u64().unwrap_or(0) as i32),
            None => Some(0)
        },
        rewards: Some(data.get("rewards").unwrap().clone()),
        skills: match data.get("skills") {
            Some(skills) => {
                let skills = skills.as_array().unwrap();
                let mut skills_vec = Vec::new();
                for skill in skills {
                    skills_vec.push(Some(skill.clone()));
                }
                Some(skills_vec)
            },
            None => None
        },
        _type: Some(data.get("type").unwrap().to_string()),
        requirements: match data.get("requirements") {
            Some(requirements) => Some(requirements.to_string()),
            None => None
        },
        totalpaymentsmade: match data.get("totalPaymentsMade") {
            Some(totalpaymentsmade) => Some(totalpaymentsmade.as_u64().unwrap_or(0) as i32),
            None => Some(0)
        },
        totalwinnersselected: match data.get("totalWinnersSelected") {
            Some(totalwinnersselected) => Some(totalwinnersselected.as_u64().unwrap_or(0) as i32),
            None => Some(0)
        },
        iswinnersannounced: match data.get("isWinnersAnnounced") {
            Some(iswinnersannounced) => Some(iswinnersannounced.as_bool().unwrap_or(false)),
            None => Some(false)
        },
        region: Some(data.get("region").unwrap().to_string()),
        pocsocials: Some(data.get("pocSocials").unwrap().to_string()),
        hackathonprize: match data.get("hackathonPrize") {
            Some(hackathonprize) => Some(hackathonprize.as_bool().unwrap_or(false)),
            None => Some(false)
        },
        timetocomplete: match data.get("timeToComplete") {
            Some(time) => match time.as_str() {
                Some(time) => Some(time.to_string()),
                None => None
            },
            None => None
        },
        winners: match data.get("winners") {
            Some(winners) => Some(winners.clone()),
            None => None   
        },
        sponsor: Some(data.get("sponsor").unwrap().clone())
    };

    let inserted_value = insert_into(posts::table)
        .values(&new_post)
        .execute(connection);

    match inserted_value {
        Ok(_) => println!("Data with id {} inserted successfully", new_post.id),
        Err(e) => {
            println!("Error inserting data: {}", e)
        },
    }

    true
}

pub async fn watch(connection: &mut PgConnection) {
    let now = Utc::now() - Days::new(5);
    let now = now.format("%Y-%m-%dT00:00:00.000Z").to_string();

    // Get the project and bounty URLs
    let project_url = EarnUrl::project_url(&now);
    let bounty_url = EarnUrl::bounty_url(&now);

    // Print the URLs
    let projects = fetch_data(&bounty_url).await;
    let bounties = fetch_data(&project_url).await;

    let total_bounties = projects.get("bounties").unwrap().as_array().unwrap().iter().chain(bounties.get("bounties").unwrap().as_array().unwrap().iter());

    for total_bountie in total_bounties {
        let added = add_data_to_database(connection, total_bountie).await;
        if !added {
            println!("Error adding data to the database {}", total_bountie);
        }
    }
}