use async_trait::async_trait;
use serde::Serialize;
use sqlx::{Executor, Postgres};
use sqlx::types::time::OffsetDateTime;
use crate::traits::traits::{Deletable, Readable, Updatable};


#[derive(Debug, Serialize)]
pub struct SwapInfoDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub free_swap: i64,
    pub used_swap: i64,
}

#[async_trait]
impl Readable for SwapInfoDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            SwapInfoDTO,
            r#"SELECT id, timestamp, free_swap, used_swap FROM swap_info"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for SwapInfoDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE swap_info SET free_swap = $1, used_swap = $2 WHERE id = $3"#,
            self.free_swap,
            self.used_swap,
            self.id,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for SwapInfoDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM swap_info WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct CpuInfoDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub usage: f32,
    pub name: String,
    pub frequency: i64,
    pub vendor_id: String
}

#[async_trait]
impl Readable for CpuInfoDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            CpuInfoDTO,
            r#"SELECT id, timestamp, usage, name, frequency as "frequency!: i64", vendor_id FROM cpu_info"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for CpuInfoDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE cpu_info SET usage = $1, name = $2, frequency = $3, vendor_id = $4 WHERE id = $5"#,
            self.usage,
            self.name,
            self.frequency,
            self.vendor_id,
            self.id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for CpuInfoDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM cpu_info WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct DiskInfoDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub name: String,
    pub total_space: i64,
    pub available_space: i64,
    pub used_space: i64,
}

#[async_trait]
impl Readable for DiskInfoDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            DiskInfoDTO,
            r#"SELECT id, timestamp, name, total_space, available_space, used_space FROM disk_info"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for DiskInfoDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE disk_info SET name = $1, total_space = $2, available_space = $3, used_space = $4 WHERE id = $5"#,
            self.name,
            self.total_space,
            self.available_space,
            self.used_space,
            self.id,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for DiskInfoDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM disk_info WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct MemoryInfoDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub total_memory_mb: i64,
    pub used_memory_mb: i64
}

#[async_trait]
impl Readable for MemoryInfoDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            MemoryInfoDTO,
            r#"SELECT id, timestamp, total_memory_mb, used_memory_mb FROM memory_info"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for MemoryInfoDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE memory_info SET total_memory_mb = $1, used_memory_mb = $2 WHERE id = $3"#,
            self.total_memory_mb,
            self.used_memory_mb,
            self.id,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for MemoryInfoDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM memory_info WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ComponentTemperaturesDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_temperature: Option<f32>,
    pub threshold_critical: Option<f32>
}

#[async_trait]
impl Readable for ComponentTemperaturesDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            ComponentTemperaturesDTO,
            r#"SELECT id, timestamp, name, temperature, max_temperature, threshold_critical FROM component_temperatures"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for ComponentTemperaturesDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE component_temperatures SET name = $1, temperature = $2, max_temperature = $3, threshold_critical = $4 WHERE id = $5"#,
            self.name,
            self.temperature,
            self.max_temperature,
            self.threshold_critical,
            self.id,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for ComponentTemperaturesDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM component_temperatures WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SystemUptimeDTO {
    pub id: i32,
    pub timestamp: OffsetDateTime,
    pub seconds: i64,
    pub minutes: i64,
    pub hours: i64,
}

#[async_trait]
impl Readable for SystemUptimeDTO {
    async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database=Postgres> + Send
    {
        let rows = sqlx::query_as!(
            SystemUptimeDTO,
            r#"SELECT id, timestamp, seconds, minutes, hours FROM system_uptime"#
        ).fetch_all(executor)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
impl Updatable for SystemUptimeDTO {
    async fn update<'e, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            r#"UPDATE system_uptime SET seconds = $1, minutes = $2, hours = $3 WHERE id = $4"#,
            self.seconds,
            self.minutes,
            self.hours,
            self.id,
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Deletable for SystemUptimeDTO {
    async fn delete<'e, E>(id: i32, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres> + Send
    {
        sqlx::query!(
            "DELETE FROM system_uptime WHERE id = $1", id
        )
            .execute(executor)
            .await?;
        Ok(())
    }
}
