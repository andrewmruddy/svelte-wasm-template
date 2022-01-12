use actix_web::{post, web, HttpResponse};
use shared::models::user::User;

#[post("/read/user")]
pub async fn user(req: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("user")  
}