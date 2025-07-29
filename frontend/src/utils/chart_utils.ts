import type {CpuInfoDTO} from "../interaces/responses/interfaces.ts";

type CpuDataRaw = CpuInfoDTO[];

export interface CpuUsagePerTimestamp {
  timestamp: string;
  [cpu: string]: number | string; // e.g. cpu0: 28.3
}

export function transformCpuData(rawData: CpuDataRaw): CpuUsagePerTimestamp[] {
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