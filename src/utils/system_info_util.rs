use sysinfo::{Disks, Disk, System, RefreshKind, CpuRefreshKind, Cpu};
use crate::data_classes::system_info::{CpuInfo, DiskInfo, MemoryInfo, Metrics, SwapInfo};

pub struct SystemInfo {
    system: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        SystemInfo { system: sys}
    }

    fn refresh(&mut self){
        self.system.refresh_all();
    }

    pub fn collect_metrics(&mut self) -> Metrics {
        self.refresh();
        Metrics {
            cpu_info: self.cpu_info(),
            memory_info: self.memory_info(),
            disk_info: self.disk_info(),
            swap_info: self.swap_info(),
        }
    }

    pub fn cpu_info(&self) -> Vec<CpuInfo> {
        self.system
            .cpus()
            .iter()
            .map(|cpu| CpuInfo{
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
                vendor_id: cpu.vendor_id().to_string()
            }).collect()
    }

    pub fn swap_info(&self) -> SwapInfo {
        let free_swap: u64= self.system.free_swap();
        let used_swap: u64 = self.system.used_swap();
        SwapInfo {
            free_swap,
            used_swap
        }
    }

    pub fn disk_info(&self) -> Vec<DiskInfo> {
        let disks = Disks::new_with_refreshed_list();
        disks
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
            }).collect()
    }

    pub fn memory_info(&self) -> MemoryInfo {
        let total_memory = self.system.total_memory() / 1024;
        let used_memory = self.system.used_memory() / 1024;
        MemoryInfo {
            total_memory_mb: total_memory,
            used_memory_mb: used_memory,
        }
    }
}