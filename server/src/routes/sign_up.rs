use actix_web::{post, get, web, HttpResponse};
use shared::models::user::User;

#[post("/sign-up")]
pub async fn sign_up(req: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("sign_up")  
}

#[get("/register/{token}")]
pub async fn register(req: web::Path<(String,)>)->HttpResponse{
    return HttpResponse::Ok().body("register") 
}