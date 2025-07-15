use std::fs::File;
use std::io::Read;
use axum::{Json, Router};
use axum::routing::{get};
use crate::data_classes::system_info::{Metrics, ComponentTemperatures};
use anyhow::{Result};
use axum::http::StatusCode;
use crate::utils::system_info_util::SystemInfo;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(system_info))
        .route("/temperatures", get(component_temperatures))
}

async fn system_info() -> Result<Json<Metrics>, (StatusCode, String)>{
    let mut system = SystemInfo::new();
    let metrics = system.collect_metrics()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(metrics))
}

async fn component_temperatures() -> Result<Json<Vec<ComponentTemperatures>>, (StatusCode, String)>{
    let mut system_util = SystemInfo::new();
    let component_temps = system_util.temperatures()
        .await
        .map_err(|e| {(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())}
    )?;
    Ok(Json(component_temps))
}

async fn disk_health() {
    // TODO GET DISKS HEALTH
}
async fn iagon_status() {
    //TODO GET IAGON NODE STATUS
}

async fn get_system_uptime() {
    let mut uptime_str = String::new();
    let mut file = File::open("/proc/uptime").expect("Unable to open /proc/uptime");
    file.read_to_string(&mut uptime_str).expect("Unable to read /proc/uptime");

}


