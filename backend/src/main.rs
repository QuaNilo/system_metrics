mod routes;
mod utils;
mod data_classes;
mod config;
mod db;

use std::env;
use tokio::net::TcpListener;
use anyhow::{Result, Context};
use crate::config::init_settings;
use crate::db::MIGRATOR;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("../.env").ok();
    tracing_subscriber::fmt::init();
    init_settings();

    let sql = db::SQL::new().await.context("Failed to create DB pool")?;
    // Run migrations before starting the server
    MIGRATOR.run(&sql.pool).await.context("Failed to run migrations")?;

    let app = routes::router();
    let address : &str = "0.0.0.0:50000";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
