use actix_web::{post, web, HttpResponse};
use crate::models::token::Token;
use crate::services::security;
use shared::nodes::user::User;
use crate::services::read;
use colored::*;

#[post("/login")]
pub async fn login(req: web::Json<User>)->HttpResponse{
    println!("{}","Loging In".green().on_white());
    println!("{}{}","Key: ".blue(),&req._key);
    let user: User;
    match read::user_by_email(&req.email).await{
        Ok(_users) => {
            if _users.len()>0{
                user = _users.first().unwrap().to_owned();
                if security::validate_password(&req.password, &user.password){
                    println!("{}{}","Success! Welcome: ".green(), &user.email.to_string().white());
                    println!("{}",Token::new(vec![("_key",&user._key)]).token.to_string().yellow().on_black());
                    return HttpResponse::Ok().json(Token::new(vec![("_key",&user._key)]));
                }else{
                    println!("{}","Invalid Password".red());
                    return HttpResponse::InternalServerError().finish()
                }
                
            }else{
                println!("{}","Could Not Find User In Database".red());
                return HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => {
            println!("{}","Server Error".black().on_yellow());
            return HttpResponse::Unauthorized().finish()
        }
    }
}

#[post("/validate")]
pub async fn validate(req: web::Json<Token>)->HttpResponse{
    println!("{}","Validate Token".green().on_white());
    println!("{}{}","Token: ".blue(),&req.token);
    println!("{}{}","Token Validity: ".blue(),Token::validate(&req.token).to_string().red());
    if Token::validate(&req.token){
        return HttpResponse::Ok().json(&req.token);
    }else{
        return HttpResponse::Unauthorized().finish()
    }
}