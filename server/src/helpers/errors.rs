use actix_web::{HttpResponse, http::StatusCode};
pub fn http_error_builder(code:u16,message:String)->HttpResponse{
    return HttpResponse::build(StatusCode::from_u16(code).unwrap()).content_type("text/html; charset=utf-8").body(String::from(message));
}