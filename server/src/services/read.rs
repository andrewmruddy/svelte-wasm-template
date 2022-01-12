use crate::shared::nodes::user::User;
use crate::services::utils;
use arangors::{Connection, AqlQuery, ClientError};
use colored::Colorize;
use serde::{de::DeserializeOwned};
use shared::nodes::project::Project;
use shared::ui::card::Card;
use shared::traits::card;

enum Query{
    COLLECTION,
    GRAPH,
    DOCUMENT
}

impl Query{
    pub fn to_string(&self, from_collection: utils::Collection, to_collection: utils::Collection, field: utils::Field, key: &str, graph: Option<utils::Graph>) -> String{
        match &self{
            Query::COLLECTION =>
            {
                println!("{}","Collection Query".red().on_white());
                return format!("{}{}{}{}{}{}{}",
                "FOR item IN ",
                    to_collection.to_string(),
                    " FILTER item.",
                    field.to_string(),
                    "==\"",
                    key,
                    "\" RETURN item"
                )
            },
            Query::DOCUMENT =>
            {
                println!("{}","Document Query".red().on_white());
                return format!("RETURN DOCUMENT(\"{}/{}\"",from_collection.to_string(),key.to_string())
            },
            Query::GRAPH => 
            {
                println!("{}","Graph Query".red().on_white());
                let graph_name: String;
                match graph{
                    Some(_graph)=>graph_name = _graph.to_string().to_string(),
                    None=>graph_name = "".to_string(),
                }
                return format!(
                    "FOR v, e, p IN 1..1 ANY '{}/{}' GRAPH {}
                        PRUNE IS_SAME_COLLECTION({}, v)
                        RETURN v._key",
                    from_collection.to_string(),
                    key.to_string(),
                    graph_name,
                    to_collection.to_string(),
                )
            }
        }
    }
}

async fn execute<T>(query: String) -> Result<Vec<T>,ClientError>
where T: DeserializeOwned,
{
    let ip = utils::get_ip();
    let conn = Connection::establish_basic_auth(ip, "root", "fCVUHKZGRA4").await.unwrap();
    let db = conn.db(utils::get_db_name()).await.unwrap();
    let aql = AqlQuery::builder()
    .query(&query)
    .count(true)
    .build();    
    let resp: Vec<T> = db.aql_query(aql).await?;
    Ok(resp)
}

pub async fn user_by_key(_key: &str) -> Result<Vec<User>,ClientError>{
    let query: String = Query::COLLECTION.to_string(utils::Collection::User,utils::Collection::User,utils::Field::KEY, _key,None);
    Ok(execute::<User>(query).await?)
}

pub async fn user_by_email(email: &str) -> Result<Vec<User>,ClientError>{
    let query: String = Query::COLLECTION.to_string(utils::Collection::User,utils::Collection::User,utils::Field::EMAIL, &email,None);
    Ok(execute::<User>(query).await?)
}

pub async fn cards<T>(_key: &str, from_collection: utils::Collection, to_collection: utils::Collection, graph: utils::Graph) -> Result<Vec<Card>, ClientError>
where T: DeserializeOwned + Clone + card::Card,
{
    let query: String = Query::GRAPH.to_string(from_collection,to_collection, utils::Field::KEY, &_key,  Some(graph));
    let keys: Vec<String> = execute::<String>(query).await?;
    let mut documents: Vec<Card> = Vec::new();
    for key in keys{
        let query: String = Query::DOCUMENT.to_string(from_collection, to_collection, utils::Field::KEY, &key, None);
        let document: T = execute::<T>(query).await?[0].clone();
        documents.push(document.to_card());
    }
    return Ok(documents);
}

pub async fn document<T>(_key: &str, collection: utils::Collection) -> Result<T, ClientError>
where T: DeserializeOwned + Clone,
{
    let query: String = Query::DOCUMENT.to_string(collection.clone(),collection.clone(), utils::Field::KEY, &_key,  None);
    let document: Vec<T> = execute::<T>(query).await?;
    return Ok(document[0].clone());
}
