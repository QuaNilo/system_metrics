export interface SwapInfoDTO {
  id: number;
  timestamp: string; // OffsetDateTime as ISO string
  free_swap: number;
  used_swap: number;
}

export interface CpuInfoDTO {
  id: number;
  timestamp: string;
  usage: number;
  name: string;
  frequency: number;
  vendor_id: string;
}

export interface DiskInfoDTO {
  id: number;
  timestamp: string;
  name: string;
  total_space: number;
  available_space: number;
  used_space: number;
}

export interface MemoryInfoDTO {
  id: number;
  timestamp: string;
  total_memory_mb: number;
  used_memory_mb: number;
}

export interface ComponentTemperaturesDTO {
  id: number;
  timestamp: string;
  name: string | null;
  temperature: number | null;
  max_temperature: number | null;
  threshold_critical: number | null;
}

export interface SystemUptimeDTO {
  id: number;
  timestamp: string;
  seconds: number;
  minutes: number;
  hours: number;
}

export interface MetricsDTO {
  cpu_info: CpuInfoDTO[];
  disk_info: DiskInfoDTO[];
  memory_info: MemoryInfoDTO[];
  swap_info: SwapInfoDTO[];
  system_uptime: SystemUptimeDTO[];
  component_temperatures: ComponentTemperaturesDTO[];
}
