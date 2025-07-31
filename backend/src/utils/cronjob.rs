use time::OffsetDateTime;
use crate::config::{get_settings, Settings};
use crate::utils::emailer::{EmailBody, MAILER};
use crate::data_classes::iagon_node::IagonNodeStatus;
use crate::data_classes::system_info::{CpuInfo, DiskInfo, MemoryInfo, SwapInfo, SystemUptime};
use crate::db::SQL;
use crate::routes::iagon::{cli_path, iagon_node_response, iagon_node_status};
use crate::routes::syscalls::get_system_uptime;
use crate::traits::traits::Creatable;
use crate::utils::system_info_util::SystemInfo;

pub async fn run_system_jobs() -> Result<(), String>{
    let batch_timestamp: OffsetDateTime = OffsetDateTime::now_utc();
    let sql = SQL::new().await.map_err(|e| e.to_string())?;
    let mut sysinfo = SystemInfo::new().await;
    match sysinfo.temperatures().await {
        Ok(temperatures) => {
            for temperature in temperatures {
                temperature.create(&sql.pool, batch_timestamp).await.map_err(|e| e.to_string())?;
            }
        },
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let metrics = match sysinfo.collect_metrics().await {
        Ok(metrics) => Ok(metrics),
        Err(e) => Err(e.to_string())
    };
    match get_system_uptime().await {
        Ok(json_uptime) => {
            let uptime: SystemUptime = json_uptime.0;
            uptime.create(&sql.pool, batch_timestamp).await.map_err(|e| e.to_string())?;
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
                    .create(&sql.pool, batch_timestamp)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            for disk_info in vec_disk_info {
                disk_info
                    .create(&sql.pool, batch_timestamp)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            swap_info
                .create(&sql.pool, batch_timestamp)
                .await
                .map_err(|e| e.to_string())?;
            memory_info
                .create(&sql.pool, batch_timestamp)
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
    let config = get_settings();
    let command = cli_path()
        .await
        .map_err(|(status, msg)| format!("cli_path failed: {}: {}", status, msg))?;
    
    verify_node_status(&config, &command).await?;
    Ok(())
}

async fn verify_node_status(config: &Settings, command: &str) -> Result<(), String>{
        let node_status: IagonNodeStatus = iagon_node_status(command)
        .await
        .map_err(|(status, msg)| format!("Node status error {}: {}", status, msg))?;
    match node_status {
        IagonNodeStatus::NodeStatusDown => {
            MAILER.send_email(
                &config.emailer.to_address,
                &"Iagon Node Down",
                &EmailBody::node_down()
            )?;
        }
        IagonNodeStatus::NodeStatusUp => {}
    }
    Ok(())
}