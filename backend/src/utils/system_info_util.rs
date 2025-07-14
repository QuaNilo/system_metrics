use sysinfo::{Disks, Disk, System, RefreshKind, CpuRefreshKind, Cpu};
use crate::data_classes::system_info::{CpuInfo, DiskInfo, MemoryInfo, Metrics, SwapInfo};
use anyhow::{Result, Context};

pub struct SystemInfo {
    system: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        SystemInfo { system: sys}
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
        let cpu_info = self.system
            .cpus()
            .iter()
            .map(|cpu| CpuInfo{
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
                vendor_id: cpu.vendor_id().to_string()
            }).collect();
        Ok(cpu_info)
    }

    pub async fn swap_info(&self) -> Result<SwapInfo, String> {
        let free_swap: u64= self.system.free_swap();
        let used_swap: u64 = self.system.used_swap();
        let swap_info = SwapInfo {
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
        let info = disks
            .list()
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
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
        let total_memory = self.system.total_memory() / 1024;
        let used_memory = self.system.used_memory() / 1024;
        let memory_info = MemoryInfo {
            total_memory_mb: total_memory,
            used_memory_mb: used_memory,
        };
        Ok(memory_info)
    }
}