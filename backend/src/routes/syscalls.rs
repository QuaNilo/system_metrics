use axum::{Json, Router};
use axum::routing::{get,post,patch,delete};
use crate::data_classes::system_info::{*};
use crate::utils::system_info_util::{*};

pub fn router() -> Router {
    Router::new()
        .route("/info", get(system_info))
}

async fn system_info() -> Json<Metrics>{
    let mut system = SystemInfo::new();
    // TODO GET COMPONENTS TEMPERATURES
    Json(system.collect_metrics())
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


