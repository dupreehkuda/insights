use crate::errors::CustomError::{CannotRenderInsights, EventAlreadyStarted, NoEventFound};
use crate::{models, service};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::env;
use std::str::FromStr;
use uuid::Uuid;

#[get("/insights/{event_id}")]
async fn new_insight(service: web::Data<service::Service>, req: HttpRequest) -> impl Responder {
    let event_id = req.match_info().get("event_id").unwrap_or_default();
    let event_id_as_uuid = Uuid::from_str(event_id).unwrap();

    let brief_event_info = service
        .get_brief_event_info(event_id_as_uuid)
        .await
        .unwrap();

    let mut body = models::NewInsightTemplate {
        event_id: event_id.to_string(),
        event_subject: brief_event_info.event_subject.clone(),
        host: env::var("SERVICE_HOST").unwrap(),
    }
    .render()
    .unwrap();

    if brief_event_info.event_subject.is_empty() {
        body = models::ErrorTemplate {
            error: NoEventFound.to_string(),
        }
        .render()
        .unwrap()
    }

    if !brief_event_info.filling {
        body = models::ErrorTemplate {
            error: EventAlreadyStarted.to_string(),
        }
        .render()
        .unwrap()
    }

    HttpResponse::Ok().body(body)
}

#[get("/summary/{event_id}")]
async fn insights_summary(
    service: web::Data<service::Service>,
    req: HttpRequest,
) -> impl Responder {
    let event_id = req.match_info().get("event_id").unwrap_or_default();
    let event_id_as_uuid = Uuid::from_str(event_id).unwrap();

    let body = match service.get_all_insights_for_event(event_id_as_uuid).await {
        Ok(insights) => insights.render().unwrap(),
        Err(error) => models::ErrorTemplate {
            error: CannotRenderInsights(error.to_string()).to_string(),
        }
        .render()
        .unwrap(),
    };

    HttpResponse::Ok().body(body)
}
