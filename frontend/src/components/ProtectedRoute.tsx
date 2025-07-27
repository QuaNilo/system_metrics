import React, { useEffect, useState } from "react";
import { Navigate } from "react-router-dom";
import {session} from "../utils/api.tsx";

const ProtectedRoute: React.FC<{ children: React.ReactElement }> = ({ children }) => {
  const [isLoading, setIsLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    async function checkSession() {
      try {
        const authorized = await session();
        setIsAuthenticated(authorized);

      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (error) {
        setIsAuthenticated(false);
      } finally {
        setIsLoading(false);
      }
    }

    checkSession();
  }, []);

  if (isLoading) {
    return <div>Loading...</div>; // or a spinner
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return children;
};

export default ProtectedRoute;