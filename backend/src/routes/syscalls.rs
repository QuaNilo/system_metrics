use std::fs::File;
use std::io::Read;
use axum::{Json, Router};
use axum::routing::{get};
use crate::data_classes::system_info::{ComponentTemperatures, SystemUptime};
use anyhow::{Result};
use axum::http::StatusCode;
use crate::data_classes::db::system_info::{ComponentTemperaturesDTO, CpuInfoDTO, DiskInfoDTO, MemoryInfoDTO, MetricsDTO, SwapInfoDTO, SystemUptimeDTO};
use crate::db::SQL;
use crate::traits::traits::Readable;
use crate::utils::system_info_util::SystemInfo;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(system_info))
        .route("/temperatures", get(component_temperatures))
        .route("/uptime", get(get_system_uptime))
}

pub async fn system_info() -> Result<Json<MetricsDTO>, (StatusCode, String)>{
    let pool = SQL::new().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?.pool;
    let interval_days: i64 = 1;
    let swap_info: Vec<SwapInfoDTO> = match SwapInfoDTO::get_latest(&pool, &interval_days).await {
        Ok(swap_info) => swap_info,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let cpu_info: Vec<CpuInfoDTO> = match CpuInfoDTO::get_latest(&pool, &interval_days).await {
        Ok(cpu_info) => cpu_info,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let disk_info: Vec<DiskInfoDTO> = match DiskInfoDTO::get_latest(&pool, &interval_days).await {
        Ok(disk_info) => disk_info,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let memory_info: Vec<MemoryInfoDTO> = match MemoryInfoDTO::get_latest(&pool, &interval_days).await {
        Ok(memory_info) => memory_info,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let component_temperatures: Vec<ComponentTemperaturesDTO> = match ComponentTemperaturesDTO::get_latest(&pool, &interval_days).await {
        Ok(component_temperature) => component_temperature,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let system_uptime: Vec<SystemUptimeDTO> = match SystemUptimeDTO::get_latest(&pool, &interval_days).await {
        Ok(system_uptime) => system_uptime,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let metrics: MetricsDTO = MetricsDTO{
        cpu_info,
        disk_info,
        memory_info,
        swap_info,
        system_uptime,
        component_temperatures,
    };
    Ok(Json(metrics))
}

pub async fn component_temperatures() -> Result<Json<Vec<ComponentTemperatures>>, (StatusCode, String)>{
    let mut system_util = SystemInfo::new().await;
    let component_temps = system_util.temperatures()
        .await
        .map_err(|e| {(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())}
    )?;
    Ok(Json(component_temps))
}

async fn disk_health() {
    // TODO GET DISKS HEALTH
}

pub async fn get_system_uptime() -> Result<Json<SystemUptime>, (StatusCode, String)> {
    let mut uptime_str = String::new();
    let mut file = File::open("/proc/uptime").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    file.read_to_string(&mut uptime_str).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let uptime_secs: i64 = uptime_str
        .split(" ")
        .next()
        .ok_or_else(||(StatusCode::INTERNAL_SERVER_ERROR, "Malformed /proc/uptime content: no space-separated values found".to_string()))?
        .parse::<f64>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? as i64;

    let uptime_mins: i64 = uptime_secs / 60;
    let uptime_hours: i64 = uptime_mins / 60;
    let system_uptime = SystemUptime{
        seconds: uptime_secs,
        minutes: uptime_mins,
        hours: uptime_hours,
    };
    Ok(Json(system_uptime))
}


