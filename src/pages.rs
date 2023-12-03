use std::str::FromStr;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use askama::Template;
use uuid::Uuid;
use crate::{models, service};
use crate::errors::CustomError::NoEventFound;

#[get("/insights/{event_id}")]
async fn new_insight(
    service: web::Data<service::Service>,
    req: HttpRequest
) -> impl Responder {
    let event_id = req.match_info().get("event_id").unwrap_or_default();
    let event_id_as_uuid = Uuid::from_str(event_id.clone()).unwrap();

    let body = match service.check_event_existence(event_id_as_uuid).await.unwrap() {
        true => models::NewInsightTemplate { event_id: event_id.to_string() }.render().unwrap(),
        false => models::ErrorTemplate { error: NoEventFound.to_string() }.render().unwrap(),
    };

    HttpResponse::Ok().body(body)
}