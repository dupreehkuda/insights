use actix_web::{get, HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::models;

#[get("/insights/{event_id}")]
async fn html_page(req: HttpRequest) -> impl Responder {
    let event_id = req.match_info().get("event_id").unwrap_or_default().to_string();

    let template = models::NewInsightTemplate { event_id };

    let body = template.render().unwrap();
    HttpResponse::Ok().body(body)
}