import type {CpuInfoDTO, MemoryInfoDTO} from "../interaces/responses/interfaces.ts";

type CpuDataRaw = CpuInfoDTO[];

export interface CpuUsagePerTimestamp {
  timestamp: string;
  [cpu: string]: number | string;
}

export interface MemoryInfoPerTimestamp {
  timestamp: string;
  total_memory_mb: number;
  used_memory_mb: number;
}

export function transformMemoryData(data: MemoryInfoDTO[]): MemoryInfoPerTimestamp[] {
    const grouped = new Map<string, MemoryInfoPerTimestamp>();
    for (const entry of data) {
    // Convert timestamp array to ISO string
    const date = new Date(
        entry.timestamp
    );

    const timeStr = date.toISOString();
    if (!grouped.has(timeStr)) {
      grouped.set(timeStr, { timestamp: timeStr, total_memory_mb: entry.total_memory_mb / 1024, used_memory_mb: entry.used_memory_mb / 1024 });
    }
  }
  return Array.from(grouped.values());
}

function transformCpuData(rawData: CpuDataRaw): CpuUsagePerTimestamp[] {
  const grouped = new Map<string, CpuUsagePerTimestamp>();

  for (const entry of rawData) {
    // Convert timestamp array to ISO string
    const date = new Date(
        entry.timestamp
    );

    const timeStr = date.toISOString();
    if (!grouped.has(timeStr)) {
      grouped.set(timeStr, { timestamp: timeStr });
    }

    grouped.get(timeStr)![entry.name] = entry.usage;
  }

  return Array.from(grouped.values());
}

export const parse_cpu_info = (
    data: CpuInfoDTO[],
): [CpuUsagePerTimestamp[], string[]] => {
    const transformedCpuData = transformCpuData(data)
    const keys = transformedCpuData.length > 0
    ? Object.keys(transformedCpuData[0]).filter(k => k !== "timestamp")
    : [];
    return [transformedCpuData.reverse(), keys.reverse()];
}