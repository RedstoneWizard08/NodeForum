<script lang="ts">
    import LeftSidebar from "$c/LeftSidebar.svelte";
    import "$s/layout.scss";

    import { onMount } from "svelte";

    onMount(() => {
        const path = "https://cdn.jsdelivr.net/npm/eruda";

        const scr = document.createElement("script");
        scr.src = path;

        scr.addEventListener("load", () => {
            const script = document.createElement("script");
            script.innerHTML = `eruda.init({ defaults: { displaySize: 30 } });`;

            document.body.appendChild(script);
        });

        document.body.appendChild(scr);
    });
</script>

<svelte:head>
    <title>NodeForum</title>
</svelte:head>

<div class="layout">
    <nav class="layout--nav">
        <a href="/" class="layout--nav--logo">NodeForum</a>
        <input class="layout--nav--search" type="text" placeholder="Search..." />

        <a href="/auth/login" class="layout--nav--link secondary">Log in</a>
        <a href="/auth/register" class="layout--nav--link primary">Register</a>
    </nav>

    <div class="layout--content">
        <LeftSidebar />

        <slot />
    </div>
</div>
