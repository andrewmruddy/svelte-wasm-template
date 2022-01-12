use serde::{Serialize, Deserialize};
use serde_json;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Route{
    Home = "#/Home",
    About = "#/About",
    Login = "#/Login",
    Register = "#/Register",
    ThankYou = "#/ThankYou",
    Projects = "#/Projects",
    Project = "#/Project",
    Dashboard = "#/Dashboard",
    Gantt = "#/Gantt"
}