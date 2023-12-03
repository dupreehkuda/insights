use bb8_postgres::bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use tokio_postgres::Error;
use uuid::Uuid;
use crate::models::{RegisterEventRequest, RegisterInsight};

#[derive(Clone)]
pub struct Postgres {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub async fn new_postgres_repository(dsn: &str) -> Result<Postgres, Error> {
    let manager = PostgresConnectionManager::new(dsn.parse()?, NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();

    Ok(Postgres { pool })
}

impl Postgres {
    pub async fn register_new_event(&self, req: RegisterEventRequest) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute("INSERT INTO insights_events (event_id, club_id) VALUES ($1, $2);", &[&req.event_id, &req.club_id])
            .await;

        result.map(|_| ())
    }

    pub async fn register_new_insight(&self, req: RegisterInsight) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute("INSERT INTO insights (insight_id, event_id, insight) VALUES ($1, $2, $3);",
                     &[&req.insight_id, &req.event_id, &req.insight])
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

    pub async fn start_event(&self, event_id: Uuid) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute("UPDATE insights_events SET filling = false WHERE event_id = $1;",
                     &[&event_id])
            .await;

        result.map(|_| ())
    }
}