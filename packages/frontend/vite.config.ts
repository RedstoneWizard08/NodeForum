import path from "path";
import { sveltekit } from "@sveltejs/kit/vite";
import type { UserConfig } from "vite";

const config: UserConfig = {
    plugins: [sveltekit()],

    resolve: {
        alias: {
            $s: "./src/styles",
            $t: "./src/types",
            $c: "./src/components",
            $r: "./src/routes",
            $d: "./src/stores",
            $api: "./src/api",
        },
    },
};

export default config;
