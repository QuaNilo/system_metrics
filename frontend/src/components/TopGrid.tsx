import React from "react";

const TopGrid: React.FC = () => {
    return (
        <>
            <div className={"grid grid-cols-3 space-x-2"}>
                <div className={"col-span-1 bg-blue-500"}>
                    test
                </div>
                <div className={"col-span-1 bg-blue-500"}>
                    test
                </div>
                <div className={"col-span-1 bg-blue-500"}>
                    test
                </div>
            </div>
        </>
    )
}

export default TopGrid;