use crate::routes::iagon::iagon_node_response;
use crate::routes::syscalls::{component_temperatures, get_system_uptime, system_info};

pub async fn run_system_jobs(){
    let temperatures = component_temperatures().await;
    // println!("{:?}", temperatures);
    let uptime = get_system_uptime().await;
    // println!("{:?}", uptime);
    let system_info = system_info().await;
    // println!("{:?}", system_info);
}

pub async fn run_iagon_jobs(){
    let node_response = iagon_node_response().await;
    // println!("{:?}", node_response);
}