use actix_web::{post, web, HttpResponse};
use shared::models::user::User;

#[post("/login")]
pub async fn login(req: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("login")  
}
