pub mod routes;
pub mod helpers;
pub mod services;
pub mod models;
extern crate shared;
extern crate colored;
use services::utils;

#[macro_use] 
extern crate serde_derive;
use actix_cors::Cors;
use actix_web::{Error,get, post,http,http::Method, web, App, HttpResponse,HttpRequest, HttpServer, Responder,http::StatusCode, dev::ServiceRequest, ResponseError,};
use actix_web_httpauth::{middleware::HttpAuthentication, extractors::{bearer::{BearerAuth, Config}, AuthenticationError}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allow_any_header()
        .max_age(3600);
        // let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(cors)
            // Auth
            .service(routes::sign_up::sign_up)
            .service(routes::sign_up::register)
            .service(routes::login::login)
            // Create
            .service(routes::create::user)
            // Read
            .service(routes::read::user)
            // Update
            .service(routes::update::user)
            // Delete
            .service(routes::delete::user)
    })
    .bind(utils::get_ip())?
    .run()
    .await
}