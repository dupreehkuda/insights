use crate::{models, service};
use actix_web::{post, web, HttpResponse, Responder};
use std::env;
use log::info;

#[post("/api/v1/event/register")]
async fn register_event(
    service: web::Data<service::Service>,
    request: web::Json<models::RegisterEventRequest>,
) -> impl Responder {
    match service.register_new_event(request.clone()).await {
        Ok(_) => {
            let host = env::var("SERVICE_HOST").unwrap();

            let response = models::RegisterEventResponse {
                insights_link: format!("{}/insights/{}", host, request.event_id),
            };

            info!("/api/v1/event/register success 200");
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);
            info!("/api/v1/event/register error 400");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/v1/event/start")]
async fn start_event(
    service: web::Data<service::Service>,
    request: web::Json<models::ManageEventRequest>,
) -> impl Responder {
    match service.start_event(request.event_id).await {
        Ok(_) => {
            let host = env::var("SERVICE_HOST").unwrap();

            let response = models::StartEventResponse {
                summary_link: format!("{}/summary/{}", host, request.event_id),
                error: None,
            };

            info!("/api/v1/event/start success 200");
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);

            let response = models::StartEventResponse {
                summary_link: String::new(),
                error: Some(err.to_string()),
            };

            info!("/api/v1/event/start error 400");
            HttpResponse::BadRequest().json(response)
        }
    }
}

#[post("/api/v1/event/finish")]
async fn finish_event(
    service: web::Data<service::Service>,
    request: web::Json<models::ManageEventRequest>,
) -> impl Responder {
    match service.finish_event(request.event_id).await {
        Ok(_) => {
            info!("/api/v1/event/finish success 200");
            HttpResponse::Ok()
        },
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);
            info!("/api/v1/event/finish error 400");
            HttpResponse::BadRequest()
        }
    }
}

#[post("/api/v1/insight")]
async fn register_insight(
    service: web::Data<service::Service>,
    request: web::Json<models::RegisterInsightRequest>,
) -> impl Responder {
    match service.register_new_insight(request.clone()).await {
        Ok(_) => {
            info!("/api/v1/insight success 200");
            HttpResponse::Ok().finish()
        },
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);
            info!("/api/v1/insight error 500");
            HttpResponse::InternalServerError().finish()
        }
    }
}
