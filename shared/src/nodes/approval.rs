use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct Approval{
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
impl Approval{
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<Approval>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}