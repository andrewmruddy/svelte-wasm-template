use serde::{Serialize, Deserialize};
use serde_json;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::traits::card::Card;

#[derive(Serialize,Deserialize, Clone)]
#[wasm_bindgen]
pub struct Project{
    _id: String,
    _key: String,
    _rev: String,
    title: String,
    number: String,
    description: String,
    business_case: String
}

#[derive(Serialize,Deserialize)]
pub enum ProjectStatus{
    Open,
    Closed,
    OnHold
}

#[wasm_bindgen]
impl Project{
    // ID
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = id)]
    pub fn _id(&self) -> String {
        self._id.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setId)]
    pub fn set__id(&mut self, _id: String) {
        self._id = _id;
    }
    // KEY
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = key)]
    pub fn _key(&self) -> String {
        self._key.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setKey)]
    pub fn set__key(&mut self, _key: String) {
        self._key = _key;
    }
    // REV
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = rev)]
    pub fn _rev(&self) -> String {
        self._rev.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setRev)]
    pub fn set__rev(&mut self, _rev: String) {
        self._rev = _rev;
    }
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
    // NUMBER
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = number)]
    pub fn number(&self) -> String {
        self.number.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setNumber)]
    pub fn set_number(&mut self, number: String) {
        self.number = number;
    }
    //DESCRIPTION
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = description)]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setDescription)]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    // BUSINESS CASE
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = businessCase)]
    pub fn business_case(&self) -> String {
        self.business_case.clone()
    }

    #[wasm_bindgen(setter)]
    #[wasm_bindgen(js_name = setBusinessCase)]
    pub fn set_business_case(&mut self, business_case: String) {
        self.business_case = business_case;
    }

    #[wasm_bindgen(constructor)]
    pub fn new()->Project{
        return Project { _id: "".to_string(), _key: "".to_string(), _rev: "".to_string(), title: "".to_string(), number: "".to_string(), description: "".to_string(), business_case: "".to_string() }
    }

    pub fn se(&self)->Option<String>{
        let serialized_result = serde_json::to_string(&self);
        match serialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
    pub fn de(string:&str)->Option<Project>{
        let deserialized_result = serde_json::from_str(&string);
        match deserialized_result{
            Ok(result) => return Some(result),
            Err(_e) => return None
        }
    }
}

impl Card for Project{
    fn to_card(&self) ->crate::ui::card::Card {
        return crate::ui::card::Card::from_project(self)
    }
}