import axios, { type AxiosResponse } from "axios";
import { endpoint, routes } from "./config";
import type { LoginRequestData, LoginResponse, RegisterRequestData } from "./types";

const login = async (data: LoginRequestData) => {
    return await axios.post(`${endpoint}${routes.login}`, data) as AxiosResponse<LoginResponse, any>;
};

const register = async (data: RegisterRequestData) => {
    return await axios.post(`${endpoint}${routes.register}`, data, { validateStatus: () => true });
};

export const api = {
    login,
    register,
};

export default api;
