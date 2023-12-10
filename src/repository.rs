use crate::models::{
    BriefEventInfo, InsightsSummaryTemplate, RegisterEventRequest, RegisterInsight,
};
use bb8_postgres::bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use tokio_postgres::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Repository {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub async fn new_postgres_repository(dsn: &str) -> Result<Repository, Error> {
    let manager = PostgresConnectionManager::new(dsn.parse()?, NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();

    Ok(Repository { pool })
}

impl Repository {
    pub async fn register_new_event(&self, req: RegisterEventRequest) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "INSERT INTO insights_events (event_id, event_subject, club_id) VALUES ($1, $2, $3);",
                &[&req.event_id, &req.event_subject, &req.club_id]
            )
            .await;

        result.map(|_| ())
    }

    pub async fn register_new_insight(&self, req: RegisterInsight) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "INSERT INTO insights (insight_id, event_id, insight) VALUES ($1, $2, $3);",
                &[&req.insight_id, &req.event_id, &req.insight],
            )
            .await;

        result.map(|_| ())
    }

    pub async fn check_event_existence(&self, event_id: Uuid) -> Result<bool, Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .query(
                "SELECT event_id FROM insights_events WHERE event_id = $1;",
                &[&event_id],
            )
            .await
            .unwrap();

        Ok(!result.is_empty())
    }

    pub async fn get_brief_event_info(&self, event_id: Uuid) -> Result<BriefEventInfo, Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .query(
                "SELECT event_subject, filling, finished FROM insights_events WHERE event_id = $1;",
                &[&event_id],
            )
            .await
            .unwrap();

        if result.is_empty() {
            return Ok(BriefEventInfo {
                event_subject: "".to_string(),
                filling: false,
                finished: false,
            });
        }

        let event_subject: String = result[0].get(0);
        let filling: bool = result[0].get(1);
        let finished: bool = result[0].get(2);

        Ok(BriefEventInfo {
            event_subject,
            filling,
            finished,
        })
    }

    pub async fn start_event(&self, event_id: Uuid) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "UPDATE insights_events SET filling = false, started_at = now() WHERE event_id = $1;",
                &[&event_id],
            )
            .await;

        result.map(|_| ())
    }

    pub async fn finish_event(&self, event_id: Uuid) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "UPDATE insights_events SET finished = true, finished_at = now() WHERE event_id = $1;",
                &[&event_id],
            )
            .await;

        result.map(|_| ())
    }

    pub async fn get_all_insights_for_event(
        &self,
        event_id: Uuid,
    ) -> Result<InsightsSummaryTemplate, Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .query(
                "SELECT insight FROM insights WHERE event_id = $1;",
                &[&event_id],
            )
            .await
            .unwrap();

        let mut ans = InsightsSummaryTemplate { insights: vec![] };

        for row in result {
            ans.insights.push(row.get(0))
        }

        Ok(ans)
    }
}
