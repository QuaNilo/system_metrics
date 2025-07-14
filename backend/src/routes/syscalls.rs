use axum::{Json, Router, response::IntoResponse};
use axum::routing::{get,post,patch,delete};
use crate::data_classes::system_info::{*};
use crate::utils::system_info_util::{*};
use anyhow::{Result, Context};
use axum::http::StatusCode;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(system_info))
}

async fn system_info() -> Result<Json<Metrics>, (StatusCode, String)>{
    let mut system = SystemInfo::new();
    // TODO GET COMPONENTS TEMPERATURES
    let metrics = system.collect_metrics()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(metrics))
}

async fn health_check() {
    // TODO GET DISKS HEALTH
}
async fn iagon_status() {
    //TODO GET IAGON NODE STATUS
}

async fn get_system_uptime() {
    // TODO GET UPTIME
}


