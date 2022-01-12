pub enum Field{
    KEY,
    EMAIL
}

impl Field{
    pub fn to_string(&self) -> &str{
        match &self{
            Field::KEY => return "_key",
            Field::EMAIL => return "email"
        }
    }
}
pub enum Graph{
    PROJECTS,
}

impl Graph{
    pub fn to_string(&self) -> &str{
        match &self{
            Graph::PROJECTS => return "Projects",
        }
    }
}
#[derive(Clone, Copy)]
pub enum Collection{
    User,
    Project,
    UserAssignment
}

impl Collection {
    pub fn to_string(&self)->String{
        match self{
            Collection::User => return "Users".to_string(),
            Collection::Project => return "Projects".to_string(),
            Collection::UserAssignment => return "UserAssignment".to_string()
        }
    }
}

pub fn get_ip()->&'static str{
    // return "http://192.168.50.178:8529";
    return "http://127.0.0.1:8529";
}

pub fn get_db_name()->&'static str{
    return "CMS";
}
