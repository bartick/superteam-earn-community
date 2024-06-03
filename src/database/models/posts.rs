use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, AsChangeset};
use serde::Serialize;
use uuid::Uuid;
use serde_json;

use crate::database::schema::posts;

#[derive(Insertable, Serialize, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: Uuid,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub deadline: Option<NaiveDateTime>,
    pub token: Option<String>,
    pub rewardamount: Option<i32>,
    pub rewards: Option<serde_json::Value>,
    pub skills: Option<Vec<Option<serde_json::Value>>>,
    pub _type: Option<String>,
    pub requirements: Option<String>,
    pub totalpaymentsmade: Option<i32>,
    pub totalwinnersselected: Option<i32>,
    pub iswinnersannounced: Option<bool>,
    pub region: Option<String>,
    pub pocsocials: Option<String>,
    pub hackathonprize: Option<bool>,
    pub timetocomplete: Option<String>,
    pub winners: Option<serde_json::Value>,
    pub sponsor: Option<serde_json::Value>
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub deadline: Option<NaiveDateTime>,
    pub token: Option<String>,
    pub rewardamount: Option<i32>,
    pub rewards: Option<serde_json::Value>,
    pub skills: Option<Vec<Option<serde_json::Value>>>,
    pub _type: Option<String>,
    pub requirements: Option<String>,
    pub totalpaymentsmade: Option<i32>,
    pub totalwinnersselected: Option<i32>,
    pub iswinnersannounced: Option<bool>,
    pub region: Option<String>,
    pub pocsocials: Option<String>,
    pub hackathonprize: Option<bool>,
    pub timetocomplete: Option<String>,
    pub winners: Option<serde_json::Value>,
    pub sponsor: Option<serde_json::Value>
}
