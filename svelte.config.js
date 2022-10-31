import adapter from "@sveltejs/adapter-auto";
import preprocess from "svelte-preprocess";

/** @type {import("@sveltejs/kit").Config} */
const config = {
    preprocess: preprocess(),

    kit: {
        adapter: adapter(),

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
