<script lang="ts">
    import { category } from "$d/state";
</script>

<svelte:head>
    {#if $category}
        <title>NodeForum | {$category.name}</title>
    {:else}
        <title>NodeForum | 404</title>
    {/if}
</svelte:head>

{#if $category}
    <p class="title">{$category.name}</p>
{:else}
    <p class="nf">Not Found!</p>
{/if}
