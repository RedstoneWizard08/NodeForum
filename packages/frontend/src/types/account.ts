export enum AccountStatus {
    LOGGED_OUT,
    LOGGED_IN,
    BLOCKED,
    LOCKED,
}

export interface AccountInfo {
    id: number;
    name: string;
    email: string;
    avatar: string;
    username: string;
}
