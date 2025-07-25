use sysinfo::{Disks, System, RefreshKind, CpuRefreshKind, Components};
use crate::data_classes::system_info::{ComponentTemperatures, CpuInfo, DiskInfo, MemoryInfo, Metrics, SwapInfo};
use anyhow::{Result};
use sqlx::PgPool;
use crate::db::SQL;

pub struct SystemInfo {
    system: System,
    pool: PgPool
}

impl SystemInfo {
    pub async fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let pool = SQL::new().await.expect("Failed to grab db session").pool;
        SystemInfo { system, pool }
    }

    async fn refresh(&mut self){
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_all();
    }

    pub async fn collect_metrics(&mut self) -> Result<Metrics, String> {
        self.refresh().await;
        let metrics = Metrics {
            cpu_info: self.cpu_info().await?,
            memory_info: self.memory_info().await?,
            disk_info: self.disk_info().await?,
            swap_info: self.swap_info().await?, 
        };
        Ok(metrics)
    }

    pub async fn cpu_info(&self) -> Result<Vec<CpuInfo>, String> {
        let cpu_info: Vec<CpuInfo> = self.system
            .cpus()
            .iter()
            .map(|cpu| CpuInfo{
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency() as i64,
                vendor_id: cpu.vendor_id().to_string(),
            }).collect();
        Ok(cpu_info)
    }

    pub async fn swap_info(&self) -> Result<SwapInfo, String> {
        let free_swap: i64 = self.system.free_swap() as i64;
        let used_swap: i64 = self.system.used_swap() as i64;
        let swap_info: SwapInfo = SwapInfo {
            free_swap,
            used_swap
        };
        Ok(swap_info)
    }

    pub async fn disk_info(&self) -> Result<Vec<DiskInfo>, String> {
        let disks = Disks::new_with_refreshed_list();
        if disks.list().is_empty() {
            return Err("No disks found or failed to read".into());
        }
        let info: Vec<DiskInfo> = disks
            .list()
            .iter()
            .map(|disk| {
                let total: i64 = disk.total_space() as i64;
                let available: i64 = disk.available_space() as i64;
                let used = total - available;

                DiskInfo{
                    name: disk.name().to_string_lossy().to_string(),
                    total_space: total,
                    available_space: available,
                    used_space: used,
                }
            }).collect();
        Ok(info)
    }

    pub async fn memory_info(&self) -> Result<MemoryInfo, String> {
        let total_memory: i64 = (self.system.total_memory() / 1024) as i64;
        let used_memory: i64 = (self.system.used_memory() / 1024) as i64;
        let memory_info = MemoryInfo {
            total_memory_mb: total_memory,
            used_memory_mb: used_memory,
        };
        Ok(memory_info)
    }

    pub async fn temperatures(&mut self) -> Result<Vec<ComponentTemperatures>> {
        self.refresh().await;
        let components = Components::new_with_refreshed_list();
        let mut component_temps = Vec::new();
        for component in &components {
            component_temps.push(ComponentTemperatures {
                name: Some(component.label().to_string()),
                threshold_critical: component.critical(),
                temperature: component.temperature(),
                max_temperature: component.max(),
            });
        }
        Ok(component_temps)
    }


}