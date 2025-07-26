import axios from 'axios';

export const API_URL = "http://localhost:50000";

interface LoginPayload {
  username: string;
  password: string;
}

export const login = async (payload: LoginPayload): Promise<boolean> =>  {
  const response = await axios.post(
      `${API_URL}/auth/login`,
      payload,
      { withCredentials: true }
  );
  return response.status >= 200 && response.status < 300;
};