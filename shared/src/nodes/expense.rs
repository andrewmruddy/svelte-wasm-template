use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct Expense{
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub name: String,
    pub category: Category,
    pub amount:u64,
}

#[derive(Serialize,Deserialize)]
pub enum Category{
    Operating,
    Capital
}
impl Expense{
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<Expense>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}