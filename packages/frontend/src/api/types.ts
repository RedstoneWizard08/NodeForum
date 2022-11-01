export interface LoginResponseData {
    name: string;
    email: string;
    avatar?: string;
    username: string;
};

export interface LoginResponse {
    code: number;
    message: string;
    data: LoginResponseData;
};

export interface RegisterResponse {
    code: number;
    message: string;
};

export interface LoginRequestData {
    username: string;
    password: string;
};

export interface RegisterRequestData {
    name: string;
    email: string;
    username: string;
    password: string;
};
