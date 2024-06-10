use chrono::{Days, Utc};
use dc_commands::worker::report::report as dcreport;
use uuid::Uuid;
use diesel::{insert_into, pg::PgConnection, r2d2::{ConnectionManager, Pool}, RunQueryDsl, query_dsl::QueryDsl, ExpressionMethods};
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::earn::constants::EarnUrl;
use database::{models::posts::{NewPost, Post}, schema::posts, helpers::handler};

pub async fn fetch_data(url: &str) -> serde_json::Value {
    // Fetch the data from the URL
    let response = reqwest::get(url).await.unwrap();

    // Get the body of the response as json
    let body = response.text().await.unwrap();
    let body = serde_json::from_str::<serde_json::Value>(&body).unwrap();

    // Return the value
    body
}

pub async fn add_data_to_database(connection: &mut PgConnection, data: &serde_json::Value) -> Option<Post> {
    
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

    // check if the data already exists
    let available_value: Option<Post> = posts::dsl::posts.filter(posts::id.eq(new_post.id)).first::<Post>(connection).ok();

    match available_value {
        Some(post) => {
            let new_post_as_post: Post = new_post.into(); // Convert new_post to type Post

            if post.ne(&new_post_as_post) {
                println!("Updating post with id {:?}", post.id);
                diesel::update(posts::dsl::posts.find(post.id))
                    .set(&new_post_as_post)
                    .execute(connection)
                    .expect("Error updating data");
            }
            None

        },
        None => {
            println!("Inserted post with id {:?}", new_post.id);
            let inseted_post = insert_into(posts::dsl::posts)
                .values(&new_post)
                .get_result::<Post>(connection)
                .expect("Error inserting data");
            Some(inseted_post)
        }
    }
}

async fn check_earn(pool: Pool<ConnectionManager<PgConnection>>) {
    let now = Utc::now() - Days::new(5);
    let now = now.format("%Y-%m-%dT00:00:00.000Z").to_string();

    // Get the project and bounty URLs
    let project_url = EarnUrl::project_url(&now);
    let bounty_url = EarnUrl::bounty_url(&now);

    // Print the URLs
    let projects = fetch_data(&bounty_url).await;
    let bounties = fetch_data(&project_url).await;

    let total_bounties = projects.get("bounties").unwrap().as_array().unwrap().iter().chain(bounties.get("bounties").unwrap().as_array().unwrap().iter());

    let mut bounty_posts: Vec<Post> = Vec::new();
    let mut project_posts: Vec<Post> = Vec::new();

    let connection: &mut PgConnection = &mut pool.get().unwrap();

    for total_bountie in total_bounties {
        let success_post = add_data_to_database(connection, total_bountie).await;
        match success_post {
            Some(post) => {
                if post._type == Some(String::from("bounty")) {
                    bounty_posts.push(post);
                } else if post._type == Some(String::from("project")) {
                    project_posts.push(post);
                } else {}
            },
            None => {}
        }
    }

    if bounty_posts.is_empty() && project_posts.is_empty() {
        return;
    }

    tokio::spawn(async move {
        dcreport(pool, bounty_posts, project_posts).await;
    });
}

pub async fn watch(pool: Pool<ConnectionManager<PgConnection>>) {

    println!("Loading current post before starting scheduler...");

    check_earn(pool.clone()).await;

    println!("Current post loaded...");
    println!("Starting scheduler...");

    let scheduler = JobScheduler::new().await.unwrap_or_else(|e| {
        panic!("Error creating scheduler: {}", e);
    });

    let schedule = "0 0 */2 * * *"; // every 2 hours

    let _ = scheduler.add(
        Job::new_async(schedule, move |_uuid, _l| {
            Box::pin({
                println!("Checking earn at {}", Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
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