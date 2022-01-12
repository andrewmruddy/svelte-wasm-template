use actix_web::{get, web, post, HttpResponse, http::StatusCode,};
use crate::helpers::url::Url;
use shared::nodes::user::User;

#[post("/update/user")]
pub async fn user(_col: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("user")  
}

#[post("/update/charter")]
pub async fn charter()->HttpResponse{
    return HttpResponse::Ok().body("charter")  
}

#[post("/update/action")]
pub async fn action()->HttpResponse{
    return HttpResponse::Ok().body("action")  
}

#[post("/update/expense")]
pub async fn expense()->HttpResponse{
    return HttpResponse::Ok().body("expense")  
}