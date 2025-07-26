use crate::data_classes::system_info::{CpuInfo, DiskInfo, MemoryInfo, SwapInfo, SystemUptime};
use crate::db::SQL;
use crate::routes::iagon::{cli_path, iagon_node_response, iagon_node_status};
use crate::routes::syscalls::get_system_uptime;
use crate::traits::traits::Creatable;
use crate::utils::system_info_util::SystemInfo;

pub async fn run_system_jobs() -> Result<(), String>{
    let mut sysinfo = SystemInfo::new().await;
    let metrics = match sysinfo.collect_metrics().await {
        Ok(metrics) => Ok(metrics),
        Err(e) => Err(e.to_string())
    };
    let sql = SQL::new().await.map_err(|e| e.to_string())?;
    match get_system_uptime().await {
        Ok(json_uptime) => {
            let uptime: SystemUptime = json_uptime.0;
            uptime.create(&sql.pool).await.map_err(|e| e.to_string())?;
        },
        Err(e) => {
            return Err("Failed to get Uptime".to_string());
        }
    };
    match metrics {
        Ok(metrics) => {
            let swap_info: SwapInfo = metrics.swap_info;
            let vec_cpu_info: Vec<CpuInfo> = metrics.cpu_info;
            let memory_info: MemoryInfo = metrics.memory_info;
            let vec_disk_info: Vec<DiskInfo> = metrics.disk_info;
            for cpu_info in vec_cpu_info {
                cpu_info
                    .create(&sql.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            for disk_info in vec_disk_info {
                disk_info
                    .create(&sql.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            swap_info
                .create(&sql.pool)
                .await
                .map_err(|e| e.to_string())?;
            memory_info
                .create(&sql.pool)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        },
        Err(e) => {
            Err("Failed to get Metrics".to_string())
        }
    }
}

pub async fn run_iagon_jobs() -> Result<(), String>{
    let _node_response = iagon_node_response()
        .await
        .map_err(|(status, msg)| format!("Node response error {}: {}", status, msg))?;

    let command = cli_path()
        .await
        .map_err(|(status, msg)| format!("cli_path failed: {}: {}", status, msg))?;

    let _node_status = iagon_node_status(command)
        .await
        .map_err(|(status, msg)| format!("Node status error {}: {}", status, msg))?;
    Ok(())
}