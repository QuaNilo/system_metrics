use tower_http::cors::{CorsLayer, Any};
use std::time::Duration;
use tokio::net::TcpListener;
use anyhow::{Result, Context};
use fern::Dispatch;
use chrono::Local;
use axum::http::method::Method;
use log::{info, error, LevelFilter};
use tokio::time;
use system_metrics::config::{get_settings, init_settings};
use system_metrics::db::SQL;
use system_metrics::db::MIGRATOR;
use system_metrics::routes;
use system_metrics::utils::cronjob::{run_system_jobs, run_iagon_jobs};

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_settings();
    let log_level = config.logging.rust_log.to_string();

    let log_file = config.logging.rust_log_file.to_string();

    let log_level = log_level.parse::<LevelFilter>()?;

    let file_logger = Dispatch::new()
        .level(log_level) // log everything to file
        .chain(fern::log_file(&log_file)?);

    // Stdout logging: info and above only
    let stdout_logger = Dispatch::new()
        .level(LevelFilter::Info) // change to `Warn` or `Error` if needed
        .chain(std::io::stdout());

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(stdout_logger)
        .chain(file_logger)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("../.env").ok();
    init_settings();
    setup_logger().expect("Failed to setup logger");
    let settings = get_settings();
    let sql = SQL::new().await.context("Failed to create DB pool")?;

    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse()?])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::ORIGIN,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION, // if needed
        ])
        .allow_credentials(true);

    info!("Running migrations...");
    MIGRATOR.run(&sql.pool).await.context("Failed to run DB migrations!")?;
    error!("Failed to run DB migrations!");

    if settings.cronjob.enabled {
        info!("Cronjobs are enabled and running with interval of {} seconds", settings.cronjob.interval);
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
    }else {
        info!("Cronjobs are disabled");
    }

    let app = routes::router().layer(cors);
    let address : &str = "0.0.0.0:50000";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn cronjobs(){
    let _ = run_system_jobs().await;
    let _ = run_iagon_jobs().await;
}
