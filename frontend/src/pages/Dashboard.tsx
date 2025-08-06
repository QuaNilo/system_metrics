import React, {useEffect, useState} from "react";
import Navbar from "../components/Navbar.tsx";
import MainGrid from "../components/grids/MainGrid.tsx";
import TopGrid from "../components/grids/TopGrid.tsx";
import {system_info} from "../utils/api.tsx";
import type {MetricsDTO} from "../interaces/responses/interfaces.ts";

const Dashboard: React.FC = () => {
    const [systemInfo, setSystemInfo] = useState<MetricsDTO>()
    useEffect(() => {
        const fetchData = async () => {
            const data = await system_info()
            setSystemInfo(data)
        }
      fetchData();
    }, []);
    return (
        <>
            {
                systemInfo ?
                <>
                    <Navbar/>
                    <TopGrid systemInfo={systemInfo}/>
                    <MainGrid systemInfo={systemInfo}/>
                </>
                :
                <>Loading...</>
            }
        </>
    );
}


export default Dashboard;