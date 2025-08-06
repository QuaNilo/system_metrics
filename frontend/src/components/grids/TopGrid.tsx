import React from "react";
import InfoBox from "../child-components/InfoBox.tsx";

const TopGrid: React.FC = () => {
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