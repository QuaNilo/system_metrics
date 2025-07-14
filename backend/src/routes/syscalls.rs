use axum::{Json, Router};
use axum::routing::{get,post,patch,delete};
use crate::data_classes::system_info::{*};
use crate::utils::system_info_util::{*};

pub fn router() -> Router {
    Router::new()
        .route("/metrics", get(metrics))
}

async fn metrics() -> Json<Metrics>{
    let mut system = SystemInfo::new();
    
    // TODO
    // GET DISKS, DISKS_USAGE, DISKS HEALTH
    // GET IAGON NODE STATUS
    // GET IAGON NODE COMMITED VALUE
    // GET TEMPERATURES
    Json(system.collect_metrics())
}