use crate::shared::nodes::user::User;
use arangors::{Connection, AqlQuery, ClientError};
use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use shared::edges::user_assignment::Role;
use shared::edges::user_assignment::UserAssignment;
use shared::nodes::project::Project;
use crate::services::utils;
use crate::services::security;
use uuid::Uuid;

async fn insert<T>(collection: utils::Collection, doc: &T) -> Result<(),ClientError>
where T: DeserializeOwned,T: Serialize,
{
    let ip = utils::get_ip();
    let conn = Connection::establish_basic_auth(ip, "root", "fCVUHKZGRA4").await.unwrap();
    let db = conn.db(utils::get_db_name()).await.unwrap();
    let mut document = serde_json::to_string(doc)?;
    document = str::replace(&document, "\\", "");
    let query = format!("{}{}{}{}","INSERT ", document, " INTO ",collection.to_string());
    println!("{}", query.green().on_black());
    let aql = AqlQuery::builder()
    .query(&query)
    .count(true)
    .build();    
    let _resp: Vec<T> = db.aql_query(aql).await?;
    Ok(())
}

pub async fn user(user: &mut User) -> Result<(),ClientError>{
    match security::hash_password(&user.password){
        Some(hashed_password)=>{
            user.password = hashed_password;
            Ok(insert(utils::Collection::User, user).await?)
        },
        None=>{
            Err(ClientError::InvalidServer("Invalid".to_string()))
        }
    }
}

pub async fn project(project: &mut Project, key: &str) -> Result<(),ClientError>{
    project.set__key(Uuid::new_v4().to_string());
    insert(utils::Collection::Project, project).await?;
    let mut user_assignment: UserAssignment = UserAssignment::new(&key,Role::Owner);
    user_assignment._key = Uuid::new_v4().to_string();
    user_assignment._to = format!("Projects/{}",project._key());
    insert(utils::Collection::UserAssignment, &user_assignment).await?;
    Ok(())
}