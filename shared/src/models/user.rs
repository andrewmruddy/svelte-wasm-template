use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct User{
    // pub _id: String,
    pub _key: String,
    // pub _rev: String,
    // pub first:String,
    // pub last:String,
    // pub title:String,
    pub email:String,
    // pub phone:String,
    // pub username:String,
    pub password:String,
}
impl User{
    pub fn new()->User{
        let uuid = Uuid::new_v4().to_string();
        return User { _key: uuid, email: "".to_string(), password: "".to_string() }
    }
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<User>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}