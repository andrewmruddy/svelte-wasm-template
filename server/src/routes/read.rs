use actix_web::{web, post, HttpResponse, get};
use colored::Colorize;
use shared::nodes::user::User;
use crate::{services::read, models::token::Token};
use crate::services::utils;

#[post("/read/user")]
pub async fn user(req: web::Json<User>)->HttpResponse{
    match read::user_by_key(&req._key).await{
        Ok(users) => return HttpResponse::Ok().json(users[0].clone()),
        Err(_) => return HttpResponse::InternalServerError().finish()
    }
}

#[get("/read/projects/{token}")]
pub async fn projects(req: web::Path<(String,)>)->HttpResponse{
    println!("{}","Retrieving Projects...".green().on_white());
    let _token: String = req.into_inner().0.clone();
    if Token::validate(&_token){
        let token: Token = Token::from_str(&_token);
        let key: String;
        match token.get_claim("_key"){
            Some(_key)=>key=_key.clone(),
            None=>{
                println!("{}","Key not found in token".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
        match read::cards(&key, utils::Collection::USER, ).await{
            Ok(projects)=>{
                for project in &projects{
                    println!("{}",&project.primary_label());
                }
                return HttpResponse::Ok().json(projects)  
            },
            Err(_)=> {
                println!("{}","Error executing query".red());
                return HttpResponse::Unauthorized().finish()
            }
        }
    }else{
        println!("{}","Invalid Registration Token".red());
        return HttpResponse::Unauthorized().finish()
    }
}

#[get("/read/project/{token}")]
pub async fn project(req: web::Path<(String,)>)->HttpResponse{
    println!("{}","Retrieving Projects...".green().on_white());
    let _token: String = req.into_inner().0.clone();
    if Token::validate(&_token){
        let token: Token = Token::from_str(&_token);
        let key: String;
        match token.get_claim("_key"){
            Some(_key)=>key=_key.clone(),
            None=>{
                println!("{}","Key not found in token".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
        match read::project(&key).await{
            Ok(project)=>{
                return HttpResponse::Ok().json(project)  
            },
            Err(_)=> {
                println!("{}","Error executing query".red());
                return HttpResponse::Unauthorized().finish()
            }
        }
    }else{
        println!("{}","Invalid Registration Token".red());
        return HttpResponse::Unauthorized().finish()
    }
}

#[post("/read/action")]
pub async fn action()->HttpResponse{
    return HttpResponse::Ok().body("action")  
}

#[post("/read/expense")]
pub async fn expense()->HttpResponse{
    return HttpResponse::Ok().body("expense")  
}