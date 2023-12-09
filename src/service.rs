use crate::errors::CustomError::NoEventFound;
use crate::models::{
    BriefEventInfo, RegisterEventRequest, RegisterInsight, RegisterInsightRequest,
};
use crate::repository::Repository;
use std::error::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Service {
    repository: Repository,
}

pub async fn new_insights_service(repo: Repository) -> Service {
    Service { repository: repo }
}

impl Service {
    pub async fn register_new_event(
        &self,
        req: RegisterEventRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.repository
            .register_new_event(req)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn register_new_insight(
        &self,
        req: RegisterInsightRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.repository
            .register_new_insight(RegisterInsight {
                insight_id: Uuid::new_v4(),
                event_id: req.event_id,
                insight: req.insight,
            })
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn check_event_existence(&self, event_id: Uuid) -> Result<bool, Box<dyn Error>> {
        self.repository
            .check_event_existence(event_id)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn get_brief_event_info(
        &self,
        event_id: Uuid,
    ) -> Result<BriefEventInfo, Box<dyn Error>> {
        self.repository
            .get_brief_event_info(event_id)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn start_event(&self, event_id: Uuid) -> Result<(), Box<dyn Error>> {
        if !self.check_event_existence(event_id).await.unwrap() {
            return Err(Box::new(NoEventFound));
        }

        self.repository
            .start_event(event_id)
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}
