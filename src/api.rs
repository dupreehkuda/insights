use std::env;
use actix_web::{HttpResponse, post, Responder, web};
use crate::{models, service};

#[post("/api/v1/event")]
async fn register_event(
    service: web::Data<service::Service>,
    request: web::Json<models::RegisterEventRequest>
) -> impl Responder {
    match service.register_new_event(request.clone()).await {
        Ok(_) => {
            let host = env::var("SERVICE_HOST").unwrap();

            let response = models::RegisterEventResponse {
                insights_link: format!("{}/insights/{}", host, request.event_id),
            };

            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/v1/insight")]
async fn register_insight(
    service: web::Data<service::Service>,
    request: web::Json<models::RegisterInsightRequest>
) -> impl Responder {
    match service.register_new_insight(request.clone()).await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            eprintln!("Error processing the event: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}