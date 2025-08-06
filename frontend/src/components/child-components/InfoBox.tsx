import React from "react";

interface InfoData {
    name: string;
    value: string;
}

const InfoBox: React.FC<InfoData> = ({name, value}) => {
    return (
        <>
            <div className={"border shadow-md rounded-md p-3 m-2"}>
                <div className={"text-lg font-bold flex justify-center"}>{value}</div>
                <div className={"font-light text-sm flex justify-center mt-2"}>{name}</div>
            </div>
        </>
    )
}

export default InfoBox;