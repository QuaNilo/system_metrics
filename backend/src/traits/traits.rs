
use sqlx::{Executor, Postgres};
use time::OffsetDateTime;

pub trait Creatable {
    async fn create<'e, E>(&self, executor: E, timestamp: OffsetDateTime) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

pub trait Readable: Sized + Send + Sync {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;

    async fn get_latest<'e, E>(executor: E, interval_days: &i64) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

pub trait Updatable {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

pub trait Deletable {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}