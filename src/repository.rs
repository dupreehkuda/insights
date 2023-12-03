use bb8_postgres::bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use tokio_postgres::Error;
use crate::models::{RegisterEventRequest, RegisterInsightRequest};

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
            .execute("INSERT INTO events (event_id, club_id) VALUES ($1, $2);", &[&req.event_id, &req.club_id])
            .await;

        result.map(|_| ())
    }

    pub async fn register_new_insight(&self, req: RegisterInsightRequest) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute("INSERT INTO insights (event_id, insight) VALUES ($1, $2);", &[&req.event_id, &req.insight])
            .await;

        result.map(|_| ())
    }
}