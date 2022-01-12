pub mod routes;
pub mod helpers;
pub mod services;
pub mod models;
extern crate shared;
extern crate colored;

#[cfg(test)]
mod tests;

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
            .service(routes::login::validate)
            .service(routes::login::login)
            // Create Documents
            // .service(routes::create::user)
            .service(routes::create::project)
            // .service(routes::create::action)
            // .service(routes::create::expense)
            // Read Documents
            .service(routes::read::projects)
            .service(routes::read::user)
            // Delete Documents
            .service(routes::delete::user)
            // Update Documents
            .service(routes::update::user)
            // Create Edges
            // Read Edges
            // Update Edges
            // Delete Edges
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}