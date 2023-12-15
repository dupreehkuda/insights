mod api;
mod errors;
mod models;
mod pages;
mod repository;
mod service;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::io::{Error, ErrorKind};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let repo = match env::var("DB_DSN") {
        Ok(dsn) => repository::new_postgres_repository(dsn.as_str()).await,
        Err(_) => {
            eprintln!("DB_DSN env variable not set");
            return Err(Error::new(ErrorKind::Other, "DB_DSN env variable not set"));
        }
    };

    let service = service::new_insights_service(repo.unwrap()).await;

    match env::var("SERVICE_ADDRESS") {
        Ok(value) => {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(service.clone()))
                    .service(api::register_event)
                    .service(api::register_insight)
                    .service(api::start_event)
                    .service(api::finish_event)
                    .service(pages::new_insight)
                    .service(pages::insights_summary)
            })
            .bind(value.to_string())?
            .run()
            .await
        }
        Err(_) => Err(Error::new(
            ErrorKind::Other,
            "SERVICE_PORT env variable not set",
        )),
    }
}
