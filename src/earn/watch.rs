use chrono::{Days, Utc};
use uuid::Uuid;
use diesel::{insert_into, pg::PgConnection, RunQueryDsl, r2d2::{Pool, ConnectionManager}};
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::{database::{models::posts::NewPost, schema::posts, helpers::handler}, earn::constants::EarnUrl};

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
        title: handler::parse_str(data, "title"),
        slug: handler::parse_str(data, "slug"),
        deadline: handler::parse_date(data, "deadline"),
        token: handler::parse_str(data, "token"),
        rewardamount: handler::parse_u64_as_i32(data, "rewardAmount"),
        rewards: handler::parse_value(data, "rewards"),
        skills: handler::parse_array(data, "skills"),
        _type: handler::parse_str(data, "type"),
        requirements: handler::parse_str(data, "requirements"),
        totalpaymentsmade: handler::parse_u64_as_i32(data, "totalPaymentsMade"),
        totalwinnersselected: handler::parse_u64_as_i32(data, "totalWinnersSelected"),
        iswinnersannounced: handler::parse_bool(data, "isWinnersAnnounced"),
        region: handler::parse_str(data, "region"),
        pocsocials: handler::parse_str(data, "pocSocials"),
        hackathonprize: handler::parse_bool(data, "hackathonPrize"),
        timetocomplete: handler::parse_str(data, "timeToComplete"),
        winners: handler::parse_value(data, "winners"),
        sponsor: handler::parse_value(data, "sponsor")
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

async fn check_earn(pool: Pool<ConnectionManager<PgConnection>>) {
    let now = Utc::now() - Days::new(5);
    let now = now.format("%Y-%m-%dT00:00:00.000Z").to_string();

    let connection: &mut PgConnection = &mut pool.get().unwrap();

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

pub async fn watch(pool: Pool<ConnectionManager<PgConnection>>) {
    let scheduler = JobScheduler::new().await.unwrap_or_else(|e| {
        panic!("Error creating scheduler: {}", e);
    });

    let schedule = "0 */6 * * * *"; // every 6 hours

    let _ = scheduler.add(
        Job::new_async(schedule, move |_uuid, _l| {
            Box::pin({
            let value = pool.clone();
            async move {
                check_earn(value).await;
            }
            })
        }).unwrap_or_else(|e| {
            panic!("Error adding job: {}", e);
        })
    ).await;

    scheduler.start().await.unwrap_or_else(|e| {
        panic!("Error starting scheduler: {}", e);
    });
}