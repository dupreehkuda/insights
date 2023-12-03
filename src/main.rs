mod models;
mod repository;
mod api;
mod pages;
mod service;
mod errors;

use std::env;
use std::io::{Error, ErrorKind};
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let repo = match env::var("DB_DSN") {
        Ok(dsn) => repository::new_postgres_repository(dsn.as_str()).await,
        Err(_) => {
            eprintln!("DB_DSN env variable not set");
            return Err(Error::new(ErrorKind::Other, "DB_DSN env variable not set"));
        }
    };

    let service = service::new_insights_service(repo.unwrap()).await;

    match env::var("SERVICE_PORT") {
        Ok(value) => HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(service.clone()))
                .service(api::register_event)
                .service(api::register_insight)
                .service(api::start_event)
                .service(pages::new_insight)
        })
            .bind(format!("127.0.0.1{}", value))?
            .run()
            .await,
        Err(_) => Err(Error::new(ErrorKind::Other, "SERVICE_PORT env variable not set"))
    }
}