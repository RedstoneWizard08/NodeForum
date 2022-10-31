import { AccountStatus, type AccountInfo } from "../types/account";
import { writable } from "svelte/store";

export const status = writable<AccountStatus>(AccountStatus.LOGGED_OUT);
export const account = writable<AccountInfo | null>(null);
