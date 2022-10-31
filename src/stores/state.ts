import type { Category } from "$t/categories";
import type { LoginRequestData } from "src/api/types";
import { writable } from "svelte/store";

export const category = writable<Category>(undefined);
export const autofill = writable<LoginRequestData>(undefined);
