use askama::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterEventRequest {
    pub event_id: Uuid,
    pub club_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterEventResponse {
    pub insights_link: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterInsightRequest {
    pub event_id: Uuid,
    pub insight: String,
}

#[derive(Template)]
#[template(path = "new_insight.html")]
pub struct NewInsightTemplate {
    pub event_id: String,
}