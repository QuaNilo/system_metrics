import React from "react";
import Navbar from "../components/Navbar.tsx";
import MainGrid from "../components/grids/MainGrid.tsx";
import TopGrid from "../components/grids/TopGrid.tsx";

const Dashboard: React.FC = () => {
    return (
        <>
            <Navbar/>
            <TopGrid/>
            <MainGrid/>
        </>
    );
}


export default Dashboard;