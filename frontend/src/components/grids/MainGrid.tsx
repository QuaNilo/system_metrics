import React, {useEffect, useState} from "react";
import {system_info} from "../../utils/api.tsx";
import Chart from "../child-components/Chart.tsx";
import {
    type CpuUsagePerTimestamp,
    type MemoryInfoPerTimestamp,
    parse_cpu_info,
    transformMemoryData
} from "../../utils/chart_utils.ts";

const MainGrid: React.FC = () => {
    const [cpuData, setCpuData] = useState<CpuUsagePerTimestamp[]>([])
    const [memoryInfo, setMemoryInfo] = useState<MemoryInfoPerTimestamp[]>([])
    const [cpuKeys, setCpuKeys] = useState<string[]>()
    const [loading, setLoading] = useState(true);

    const fetch_system_info = async () => {
        try{
            setLoading(true);
            const data = await system_info()
            if (!data.cpu_info) {
                return;
            }
            const [cpu_data, cpu_keys] = parse_cpu_info(data.cpu_info);
            setCpuKeys(cpu_keys);
            setCpuData(cpu_data);
            const memory_data = transformMemoryData(data.memory_info)
            setMemoryInfo(memory_data);


        }finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        const fetchData = async () => {
                await fetch_system_info();
        }
      fetchData();
    }, []);

    return (
        <>
            {loading ? <div>Loading...</div>: (
                <div className={"grid grid-cols-6 grid-rows-4 space-x-2"}>
                    {/*// CPU USAGE OVER TIME*/}
                    <div className={"col-span-6"}>
                        <Chart
                          data={cpuData}
                          xAxisKey="timestamp"
                          dataKeys={cpuKeys ?? []}
                          chartTitle="CPU Usage Over Time"
                        />
                    </div>

                    <div className={"col-span-6 row-span-1"}>
                        <Chart
                          data={memoryInfo}
                          xAxisKey="timestamp"
                          dataKeys={['total_memory_mb', 'used_memory_mb']}
                          chartTitle="Memory Usage Over Time"
                        />
                    </div>
                </div>

            )}
        </>
    )
}

export default MainGrid;