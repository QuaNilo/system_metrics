import React, { useState } from 'react'
import {login} from "../utils/api.tsx";
import { useNavigate } from "react-router-dom";
interface LoginForm {
    username: string;
    password: string;
}

const Login: React.FC = () => {
    const navigate = useNavigate();
    const [formData, setFormData] = useState<LoginForm>({
        username: "",
        password: "",
    })

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData(prev => ({ ...prev, [name]: value }));
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        console.log(formData);
        const authorized = await login(formData);
        if (authorized) {
            navigate("/dashboard");
        }else {
            alert("Invalid credentials");
        }
    };

    return (
          <div className="flex h-screen w-screen items-center justify-center bg-gray-100">
              <form
                    onSubmit={handleSubmit}
                    className="bg-white p-8 rounded shadow-md w-96 space-y-4"
                  >
                    <h2 className="text-xl font-semibold text-center">Login</h2>

                    <div>
                      <label htmlFor="username" className="block text-sm font-medium">
                        Username
                      </label>
                      <input
                        type="username"
                        name="username"
                        value={formData.username}
                        onChange={handleChange}
                        required
                        className="mt-1 block w-full border border-gray-300 rounded p-2"
                      />
                    </div>

                    <div>
                      <label htmlFor="password" className="block text-sm font-medium">
                        Password
                      </label>
                      <input
                        type="password"
                        name="password"
                        value={formData.password}
                        onChange={handleChange}
                        required
                        className="mt-1 block w-full border border-gray-300 rounded p-2"
                      />
                    </div>

                    <button
                      type="submit"
                      className="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700"
                    >
                      Login
                    </button>
              </form>
          </div>
    )
};

export default Login;