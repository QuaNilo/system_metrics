import React from "react";
import {logout} from "../utils/api.tsx";
import {useNavigate} from "react-router-dom";

const Navbar: React.FC = () => {
    const navigate = useNavigate();
    const onLogout = async (e: React.MouseEvent<HTMLButtonElement>) => {
        e.preventDefault();
        const success = await logout();
        if (success) {
            navigate("/login");
        }else {
            alert("Logout failed");
        }
    }

    return (
        <>
            <nav id={"nav"} className={"w-full px-4 py-2 bg-white shadow-md flex justify-between items-center"}>
                {/*INSERT LOGO*/}
                <div id={"app"} className={"text-xl font-bold"}>
                    MyApp
                </div>
                <div id={"tabs"} className={"hidden md:flex gap-6"}>
                    <a href={"/dashboard"} className={"hover:text-blue-600"}>Home</a>
                    <a href={"/dashboard"} className={"hover:text-blue-600"}>About</a>
                    <a href={"/dashboard"} className={"hover:text-blue-600"}>Contact</a>
                </div>
                <div id={"options"} className={"flex gap-2 items-center"}>
                    <button
                        className={"px-4 py-1 border rounded"}
                        onClick={(e) => {
                            onLogout(e);
                        }}
                    >Logout</button>
                </div>
            </nav>
        </>
    )
}

export default Navbar;