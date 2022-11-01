<script lang="ts">
    import "@fortawesome/fontawesome-free/scss/fontawesome.scss";

    import "@fortawesome/fontawesome-free/scss/solid.scss";
    import "@fortawesome/fontawesome-free/scss/brands.scss";
    import "@fortawesome/fontawesome-free/scss/regular.scss";

    import "$s/components/LeftSidebar.scss";

    import { status } from "$d/account";
    import { categories } from "$d/categories";
    import { AccountStatus } from "$t/account";
    import { category } from "$d/state";
    import { goto } from "$app/navigation";
    import type { Category } from "$t/categories";

    let categoryRefs: { [key: string]: HTMLDivElement } = {};

    const updateCategory = (c: Category) => {
        $category = c;

        goto(`/category/${c.id}`);
        document.title = `NodeForum | ${c.name}`;

        for (const key in categoryRefs) {
            const el = categoryRefs[key];

            if (el.classList.contains("active")) el.classList.remove("active");
        }

        categoryRefs[c.id].classList.add("active");
    };

    const getCategoryClasses = (c: Category) => {
        if ($category && c.id == $category.id) {
            return "sidebar--categories--item active";
        } else {
            return "sidebar--categories--item";
        }
    };
</script>

<div class="sidebar">
    {#if $status !== AccountStatus.LOGGED_IN}
        <div class="sidebar--login">
            <p class="sidebar--login--title">NodeForum</p>

            <a href="/auth/login" class="sidebar--login--link secondary">Log in</a>
            <a href="/auth/register" class="sidebar--login--link primary">Register</a>
        </div>
    {/if}

    <div class="sidebar--categories">
        {#each $categories as category}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
                class="sidebar--categories--item"
                on:click={() => updateCategory(category)}
                bind:this={categoryRefs[category.id]}
            >
                <i class={category.icon} />
                <p class="sidebar--categories--title">{category.name}</p>
            </div>
        {/each}
    </div>
</div>
