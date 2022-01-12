use actix_web::{post, web, HttpResponse};
use shared::models::user::User;

#[post("/create/user")]
pub async fn user(_col: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("user")  
}