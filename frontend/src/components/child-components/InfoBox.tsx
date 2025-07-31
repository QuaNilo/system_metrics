import React from "react";

interface InfoData {
    name: string;
    value: string;
}

const InfoBox: React.FC = (data: InfoData) => {
    return (
        <>
            <div className={"border shadow-md rounded-md p-3 m-2"}>
                <div className={"text-lg font-bold flex justify-center"}>{data.value}</div>
                <div className={"font-light text-sm flex justify-center mt-2"}>{data.name}</div>
            </div>
        </>
    )
}

export default InfoBox;