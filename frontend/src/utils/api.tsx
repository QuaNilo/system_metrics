import axios from 'axios';
import type {MetricsDTO} from "../interaces/responses/interfaces.ts";
import config from "../config.ts";

interface LoginPayload {
  username: string;
  password: string;
}

export const login = async (payload: LoginPayload): Promise<boolean> =>  {
  const response = await axios.post(
      `${config.apiUrl}/auth/login`,
      payload,
      { withCredentials: true }
  );
  return response.status >= 200 && response.status < 300;
};

export const session = async (): Promise<boolean> => {
  const response = await axios.post(`${config.apiUrl}/auth/session`, {}, { withCredentials: true });
  return response.status >= 200 && response.status < 300;
};

export const logout = async (): Promise<boolean> => {
  const response = await axios.post(`${config.apiUrl}/auth/logout`, {}, { withCredentials: true });
  return response.status >= 200 && response.status < 300;
};

export const system_info = async (): Promise<MetricsDTO> => {
  const response = await axios.get(`${config.apiUrl}/system/info`);
  if (response.status >= 200 && response.status < 300) {
    return response.data;
  } else {
    throw new Error("Failed to fetch system info");
  }
}