use actix_web::{get, post,http, web, App, HttpResponse, HttpServer, Responder,http::StatusCode,};
use crate::helpers::url::Url;
use shared::nodes::user::User;


#[post("/delete/user")]
pub async fn user(_col: web::Json<User>)->HttpResponse{
    return HttpResponse::Ok().body("user")  
}

#[post("/delete/charter")]
pub async fn charter()->HttpResponse{
    return HttpResponse::Ok().body("charter")  
}

#[post("/delete/action")]
pub async fn action()->HttpResponse{
    return HttpResponse::Ok().body("action")  
}

#[post("/delete/expense")]
pub async fn expense()->HttpResponse{
    return HttpResponse::Ok().body("expense")  
}