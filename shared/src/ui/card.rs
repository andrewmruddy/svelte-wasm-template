use serde::{Serialize, Deserialize};
use serde_json;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::nodes::project::Project;


#[derive(Serialize,Deserialize, Clone)]
#[wasm_bindgen]
pub struct Card{
    title: String,
    key: String,
    description: String,
    primary_label: String,
}

#[wasm_bindgen]
impl Card{
    // TITLE
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = title)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setTitle)]
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    // KEY
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = key)]
    pub fn key(&self) -> String {
        self.key.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setKey)]
    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }
    // DESCRIPTION
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = description)]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setId)]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    // PRIMARY_LABEL
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = primaryLabel)]
    pub fn primary_label(&self) -> String {
        self.primary_label.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setPrimaryLabel)]
    pub fn set_primary_label(&mut self, primary_label: String) {
        self.primary_label = primary_label;
    }
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Card{
        return Card { title: "".to_string(), key: "".to_string(), description: "".to_string(), primary_label: "".to_string() }
    }
    
    pub fn from_project(project: &Project) -> Card{
        return Card{
            title: project.title(),
            key: project._key(),
            description: project.description(),
            primary_label: "VIEW".to_string(),
        }
    }
    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<Card>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}