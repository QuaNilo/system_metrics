import React, {useEffect, useState} from "react";
import {system_info} from "../../utils/api.tsx";
import Chart from "../child-components/Chart.tsx";
import {type CpuUsagePerTimestamp, transformCpuData} from "../../utils/chart_utils.ts";

const MainGrid: React.FC = () => {
    const [cpuData, setCpuData] = useState<CpuUsagePerTimestamp[]>([])
    const [cpuKeys, setCpuKeys] = useState<string[]>()
    const [loading, setLoading] = useState(true);

    const fetch_system_info = async () => {
        try{
            setLoading(true);
            const data = await system_info()
            if (!data.cpu_info) {
                return;
            }

            const transformedCpuData = transformCpuData(data.cpu_info)
            setCpuData(transformedCpuData.reverse());
            const keys = transformedCpuData.length > 0
            ? Object.keys(transformedCpuData[0]).filter(k => k !== "timestamp")
            : [];
            setCpuKeys(keys.reverse())

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
                <div className={"grid grid-cols-6 space-x-2"}>
                    {/*// CPU USAGE OVER TIME*/}
                    <div className={"col-span-6"}>
                        <Chart
                          data={cpuData}
                          xAxisKey="timestamp"
                          dataKeys={cpuKeys ?? []}
                          chartTitle="CPU Usage Over Time"
                        />
                    </div>

                    <div className={"col-span-3 bg-blue-500"}>
                        test
                    </div>
                </div>

            )}
        </>
    )
}

export default MainGrid;