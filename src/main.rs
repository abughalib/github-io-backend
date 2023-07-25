pub mod database;
pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate serde;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("abu-github-io-server.eastus.cloudapp.azure.com")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096))
            .service(routes::index)
            .service(web::resource("/message").route(web::post().to(routes::message)))
        //.service(web::resource("/message").route(web::post().to(routes::message_mongoasync)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
