use askama::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterEventRequest {
    pub event_id: Uuid,
    pub event_subject: String,
    pub club_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterEventResponse {
    pub insights_link: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ManageEventRequest {
    pub event_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct StartEventResponse {
    pub summary_link: String,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterInsightRequest {
    pub event_id: Uuid,
    pub insight: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterInsight {
    pub insight_id: Uuid,
    pub event_id: Uuid,
    pub insight: String,
}

#[derive(Deserialize, Serialize)]
pub struct BriefEventInfo {
    pub event_subject: String,
    pub filling: bool,
    pub finished: bool,
}

#[derive(Template)]
#[template(path = "new_insight.html")]
pub struct NewInsightTemplate {
    pub event_id: String,
    pub event_subject: String,
    pub host: String,
}

#[derive(Template)]
#[template(path = "insights.html")]
pub struct InsightsSummaryTemplate {
    pub insights: Vec<String>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub error: String,
}
