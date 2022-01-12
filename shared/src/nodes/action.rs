use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct Action{
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub title: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
}

// impl Action{
//     pub fn new()->Action{
//         return Action{
//             title: "".to_string(),
//             description: "".to_string(),
//             start_date: Utc::now(),
//             end_date: Utc::now(),
//         }
//     }
// }

impl Action{
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<Action>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}