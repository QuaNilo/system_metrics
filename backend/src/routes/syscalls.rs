use std::fs::File;
use std::io::Read;
use std::time::Duration;
use axum::{Json, Router};
use axum::routing::{get};
use crate::data_classes::system_info::{Metrics, ComponentTemperatures, SystemUptime};
use anyhow::{Context, Error, Result};
use axum::http::StatusCode;
use tokio::io::AsyncReadExt;
use crate::utils::system_info_util::SystemInfo;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(system_info))
        .route("/temperatures", get(component_temperatures))
        .route("/uptime", get(get_system_uptime))
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

async fn get_system_uptime() -> Result<Json<SystemUptime>, (StatusCode, String)> {
    let mut uptime_str = String::new();
    let mut file = File::open("/proc/uptime").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    file.read_to_string(&mut uptime_str).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let uptime_secs = uptime_str
        .split(" ")
        .next()
        .ok_or_else(||(StatusCode::INTERNAL_SERVER_ERROR, "Malformed /proc/uptime content: no space-separated values found".to_string()))?
        .parse::<f64>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? as u64;

    let uptime_mins = uptime_secs / 60;
    let uptime_hours = uptime_mins / 60;
    println!("Uptime: \n {uptime_secs:#?} \n {uptime_mins:#?} \n {uptime_hours:#?}", uptime_secs=uptime_secs, uptime_mins=uptime_mins, uptime_hours=uptime_hours);
    Ok(Json(SystemUptime{
        seconds: uptime_secs,
        minutes: uptime_mins,
        hours: uptime_hours,
    }))
}


