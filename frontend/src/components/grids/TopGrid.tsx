import React from "react";
import InfoBox from "../child-components/InfoBox.tsx";
import type {MetricsDTO} from "../../interaces/responses/interfaces.ts";

interface TopGridProps {
    systemInfo: MetricsDTO;
}

const TopGrid: React.FC<TopGridProps> = ({systemInfo}) => {


    return (
        <>
            <div className={"grid grid-cols-6 space-x-2"}>
                <div className={"col-span-2"}>
                    {/*UPTIME*/}
                    <InfoBox name={"CPU Usage"} value={"28.3%"}/>
                </div>
                <div className={"col-span-2"}>
                    <InfoBox name={"CPU Usage"} value={"28.3%"}/>
                </div>
                <div className={"col-span-2"}>
                    <InfoBox name={"CPU Usage"} value={"28.3%"}/>
                </div>

            </div>
        </>
    )
}

export default TopGrid;