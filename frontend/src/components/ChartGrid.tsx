import React from "react";

const ChartGrid: React.FC = () => {
    return (
        <>
            <div className={"grid grid-cols-3"}>
                <div className={"col-span-1 bg-red-500"}>chart</div>
                <div className={"col-span-1 bg-red-500"}>chart</div>
                <div className={"col-span-1 bg-red-500"}>chart</div>
            </div>
        </>
    )
}

export default ChartGrid;