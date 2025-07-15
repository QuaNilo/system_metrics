use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Metrics {
    pub cpu_info: Vec<CpuInfo>,
    pub disk_info: Vec<DiskInfo>,
    pub memory_info: MemoryInfo,
    pub swap_info: SwapInfo,
}

#[derive(Debug, Serialize)]
pub struct SwapInfo {
    pub free_swap: u64,
    pub used_swap: u64,
}

#[derive(Debug, Serialize)]
pub struct CpuInfo {
    pub usage: f32,
    pub name: String,
    pub frequency: u64,
    pub vendor_id: String
}

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
}
#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total_memory_mb: u64,
    pub used_memory_mb: u64
}

#[derive(Debug, Serialize)]
pub struct ComponentTemperatures {
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_temperature: Option<f32>,
    pub threshold_critical: Option<f32>
}

#[derive(Debug, Serialize)]
pub struct SystemUptime {
    pub seconds: u64,
    pub minutes: u64,
    pub hours: u64,
}