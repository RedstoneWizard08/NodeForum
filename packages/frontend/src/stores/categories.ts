import type { Category } from "../types/categories";
import { get, writable } from "svelte/store";
import { category } from "./state";

export const categories = writable<Category[]>([
    {
        icon: "fa-solid fa-house",
        name: "General",
        id: "general",
    },
    {
        icon: "fa-solid fa-wrench",
        name: "Tools",
        id: "tools",
    },
    {
        icon: "fa-solid fa-terminal",
        name: "Development",
        id: "development",
    },
]);
