use actix_web::{post, get, web, HttpResponse};
use crate::models::token::Token;
use crate::services::{security, create};
use shared::nodes::user::User;
use crate::services::read;
use colored::*;

#[post("/sign-up")]
pub async fn sign_up(req: web::Json<User>)->HttpResponse{
    println!("{}","Sign Up".green().on_white());
    match read::user_by_email(&req.email).await{
        Ok(_users) => {
            if _users.len()>0{
                println!("{}","User already exists".red());
                return HttpResponse::Unauthorized().finish()
            }else{
                if security::validate_sign_up(&req){
                    println!("{}{}","Success! Welcome: ".green(), &req.email.to_string().white());
                    let token: Token = Token::new(vec![("email",&req.email),("password",&req.password)]);
                    println!("{}",token.token);
                    return HttpResponse::Ok().finish();
                }else{
                    println!("{}","User failed sign up validation".red());
                    return HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => {
            println!("{}","Server Error".black().on_yellow());
            return HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/register/{token}")]
pub async fn register(req: web::Path<(String,)>)->HttpResponse{
    println!("{}","Register".green().on_white());
    let _token: String = req.into_inner().0.clone();
    if Token::validate(&_token){
        let token: Token = Token::from_str(&_token);
        let mut user: User = User::new();
        match token.get_claim("email"){
            Some(email)=>user.email=email,
            None=>{
                println!("{}","Email not found in token".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
        match token.get_claim("password"){
            Some(password)=>user.password=password,
            None=>{
                println!("{}","Password not found in token".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
        match read::user_by_email(&user.email).await{
            Ok(_users) => {
                if _users.len() > 0{
                    println!("{}","User already exists".red());
                    return HttpResponse::Unauthorized().finish()
                }else{
                    match create::user(&mut user).await{
                        Ok(_users) => {
                            println!("{}{}","Successfully registered: ".green(), &user.email.to_string().white());
                            return HttpResponse::Ok().finish()
                        },
                        Err(_) => {
                            println!("{}","Server Error".black().on_yellow());
                            return HttpResponse::InternalServerError().finish()
                        }
                    }
                }
            },
            Err(_) => {
                println!("{}","Server Error".black().on_yellow());
                return HttpResponse::InternalServerError().finish()
            }
        }
    }else{
        println!("{}","Invalid Registration Token".red());
        return HttpResponse::Unauthorized().finish()
    }
}