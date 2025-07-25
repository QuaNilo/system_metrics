use sqlx::{Executor, Postgres};
use async_trait::async_trait;

#[async_trait]
pub trait Creatable {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

#[async_trait]
pub trait Readable: Sized + Send + Sync {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

#[async_trait]
pub trait Updatable {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}

#[async_trait]
pub trait Deletable {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send;
}