use async_trait::async_trait;
use serde::Serialize;
use sqlx::{Executor, Postgres};
use crate::traits::traits::Creatable;

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

#[async_trait]
impl Creatable for SwapInfo {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO swap_info (free_swap, used_swap) VALUES ($1, $2)"#,
            self.free_swap,
            self.used_swap,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct CpuInfo {
    pub usage: f32,
    pub name: String,
    pub frequency: u64,
    pub vendor_id: String
}

#[async_trait]
impl Creatable for CpuInfo {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO cpu_info (usage, name, frequency, vendor_id) VALUES ($1, $2, $3, $4)"#,
            self.usage,
            self.name,
            self.frequency,
            self.vendor_id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
}

#[async_trait]
impl Creatable for DiskInfo {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO disk_info (name, total_space, available_space, used_space) VALUES ($1, $2, $3, $4)"#,
            self.name,
            self.total_space,
            self.available_space,
            self.used_space
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}
    
#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total_memory_mb: u64,
    pub used_memory_mb: u64
}

#[async_trait]
impl Creatable for MemoryInfo {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO memory_info (total_memory_mb, used_memory_mb) VALUES ($1, $2)"#,
            self.total_memory_mb,
            self.used_memory_mb,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ComponentTemperatures {
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_temperature: Option<f32>,
    pub threshold_critical: Option<f32>
}

#[async_trait]
impl Creatable for ComponentTemperatures {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO component_temperatures (name, temperature, max_temperature, threshold_critical) VALUES ($1, $2, $3, $4)"#,
            self.name,
            self.temperature,
            self.max_temperature,
            self.threshold_critical,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SystemUptime {
    pub seconds: u64,
    pub minutes: u64,
    pub hours: u64,
}

#[async_trait]
impl Creatable for SystemUptime {
    async fn create<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        sqlx::query!(r#"INSERT INTO system_uptime (seconds, minutes, hours) VALUES ($1, $2, $3)"#,
            self.usage,
            self.name,
            self.frequency,
            self.vendor_id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}