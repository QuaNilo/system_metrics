use sqlx::PgPool;
use crate::config::get_settings;
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::Migrator;

pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct SQL{
   pub pool: PgPool,
}

impl SQL {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let settings = get_settings();
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(settings.postgres.connection_string.as_str())
            .await?;
        Ok(SQL {pool})
    }
}