import React from "react";
import Navbar from "../components/Navbar.tsx";
import MainGrid from "../components/grids/MainGrid.tsx";

const Dashboard: React.FC = () => {
    return (
        <>
            <Navbar/>
            <MainGrid/>
        </>
    );
}


export default Dashboard;