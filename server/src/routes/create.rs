use actix_web::{post, web, HttpResponse};
use colored::Colorize;
use shared::nodes::project::Project;
use crate::models::token::Token;
use crate::services::create;
use shared::nodes::user::User;
use crate::services::read;
use crate::models::req::UserReq;

#[post("/create/user")]
pub async fn user(req: web::Json<UserReq>)->HttpResponse{
    println!("{}","Create User");
    let exists: bool;
    match read::user_by_email(&req.user.email).await{
        Ok(_users) => {
            if _users.len()>0{
                exists = true;
            }else{
                exists = false;
            }
        },
        Err(_) => exists = false
    }
    if exists{
        return HttpResponse::NoContent().finish()
    }else{
        let mut user: User = req.user.to_owned();
        match create::user(&mut user).await{
            Ok(_users) => return HttpResponse::Ok().json("Done."),
            Err(_) => return HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/create/project/{token}")]
pub async fn project(project: web::Json<Project>, token: web::Path<(String,)>)->HttpResponse{
    let _token: String = token.into_inner().0.clone();
    let mut project: Project = project.into_inner().clone();
    if Token::validate(&_token){
        let token: Token = Token::from_str(&_token);
        let user_key: String;
        match token.get_claim("_key"){
            Some(key)=>user_key = key,
            None=>{
                println!("{}","Error extracting user key".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
        match create::project(&mut project, &user_key).await{
            Ok(_users) => {
                println!("{}{}","Successfully created project: ".green(), project._key().white());
                return HttpResponse::Ok().finish()
            },
            Err(_) => {
                println!("{}","Server Error".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
    }else{
        println!("{}","Invalid Auth Token".red());
        return HttpResponse::Unauthorized().finish()
    }
}

#[post("/create/action")]
pub async fn action()->HttpResponse{
    return HttpResponse::Ok().body("action")  
}

#[post("/create/expense")]
pub async fn expense()->HttpResponse{
    return HttpResponse::Ok().body("expense")  
}