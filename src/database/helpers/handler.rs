use serde_json::Value;
use chrono::NaiveDateTime;

pub fn parse_str(data: &Value, key: &str) -> Option<String> {
    match data.get(key) {
        Some(value) => Some(value.to_string()),
        None => None
    }
}

pub fn parse_date(data: &Value, key: &str) -> Option<NaiveDateTime> {
    match data.get(key) {
        Some(value) => match value.as_str() {
            Some(value) => match NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.3fZ") {
                Ok(value) => Some(value),
                Err(_) => None
            },
            None => None
        },
        None => None
    }
}
pub fn parse_bool(data: &Value, key: &str) -> Option<bool> {
    match data.get(key) {
        Some(value) => Some(value.as_bool().unwrap_or(false)),
        None => None
    }
}

pub fn parse_u64_as_i32(data: &Value, key: &str) -> Option<i32> {
    match data.get(key) {
        Some(value) => Some(value.as_u64().unwrap_or(0) as i32),
        None => Some(0)
    }
}

pub fn parse_value(data: &Value, key: &str) -> Option<Value> {
    match data.get(key) {
        Some(value) => Some(value.clone()),
        None => None
    }
}

pub fn parse_array(data: &Value, key: &str) -> Option<Vec<Option<Value>>> {
    match data.get(key) {
        Some(value) => {
            match value.as_array() {
                Some(value) => {
                    let mut value_vec = Vec::new();
                    for val in value {
                        value_vec.push(Some(val.clone()));
                    }
                    Some(value_vec)
                },
                None => None
            }
        },
        None => None
    }
}