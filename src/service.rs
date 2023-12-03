use std::error::Error;
use crate::models::{RegisterEventRequest, RegisterInsightRequest};
use crate::repository::Postgres;

#[derive(Clone)]
pub struct Service {
    repository: Postgres,
}

pub async fn new_insights_service(repo: Postgres) -> Service {
    Service {
        repository: repo,
    }
}

impl Service {
    pub async fn register_new_event(&self, req: RegisterEventRequest) -> Result<(), Box<dyn Error>> {
        self.repository
            .register_new_event(req)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn register_new_insight(&self, req: RegisterInsightRequest) -> Result<(), Box<dyn Error>> {
        self.repository
            .register_new_insight(req)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}