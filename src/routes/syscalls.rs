use axum::{Json, Router};
use axum::routing::{get,post,patch,delete};
use sysinfo;
use serde::Serialize;

pub fn router() -> Router {
    Router::new()
        .route("/metrics", get(metrics))
}

#[derive(Serialize)]
struct Metrics {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

async fn metrics() -> Json<Metrics>{
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_usage();
    println!("CPU usage: {}", cpu_usage);
    let total_memory = sys.total_memory();
    println!("Total memory: {}", total_memory);
    let used_memory = sys.used_memory();
    println!("Used memory: {}", used_memory);
    let total_swap = sys.total_swap();
    println!("Total swap: {}", total_swap);
    let used_swap = sys.used_swap();
    println!("Used swap: {}", used_swap);

    // TODO
    // GET DISKS, DISKS_USAGE, DISKS HEALTH
    // GET IAGON NODE STATUS
    // GET IAGON NODE COMMITED VALUE
    // GET TEMPERATURES

    Json(Metrics{
        cpu_usage,
        total_memory,
        used_memory,
        total_swap,
        used_swap
    })
}