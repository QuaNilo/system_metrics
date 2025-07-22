mod routes;
mod utils;
mod data_classes;
mod config;
mod db;

use std::time::Duration;
use tokio::net::TcpListener;
use anyhow::{Result, Context};
use tokio::time;
use crate::config::{get_settings, init_settings};
use crate::db::MIGRATOR;
use crate::utils::cronjob::{run_system_jobs, run_iagon_jobs};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("../.env").ok();
    tracing_subscriber::fmt::init();
    init_settings();
    let settings =get_settings();

    let sql = db::SQL::new().await.context("Failed to create DB pool")?;
    // Run migrations before starting the server
    MIGRATOR.run(&sql.pool).await.context("Failed to run migrations")?;

    tokio::spawn(async {
        let mut interval = time::interval(Duration::from_secs(settings.cronjob.interval));
        loop {
            interval.tick().await;
            tokio::spawn(
                async {
                    cronjobs().await;
                }
            );
        }
    });

    let app = routes::router();
    let address : &str = "0.0.0.0:50000";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// TODO Create database tables
// TODO Create logic to scrape metrics every X secs

async fn cronjobs(){
    // println!("Running cronjobs");
    run_system_jobs().await;
    run_iagon_jobs().await;
}