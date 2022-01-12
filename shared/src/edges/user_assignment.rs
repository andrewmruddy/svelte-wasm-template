use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct UserAssignment{
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub _from: String,
    pub _to: String,
    pub role: Role
}

#[derive(Serialize,Deserialize, Clone)]
pub enum Role{
    Owner
}

impl UserAssignment{
    pub fn new(key: &str, role: Role) -> UserAssignment{
        return UserAssignment{
            _id: "1".to_string(),
            _key: "1".to_string(),
            _rev: "1".to_string(),
            _from: format!("{}{}","Users/".to_string(),key.to_string()),
            _to: "1".to_string(),
            role: role,
        }
    }
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<UserAssignment>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}