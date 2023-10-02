<script lang="ts">
    import ModListWindow from "../components/ModListWindow.svelte";
    import LocalMods from "../components/LocalMods.svelte";

    let page = "explore"

    const switchPage = () => {
        if (page == "explore") {
            page = "local"
        } else if (page == "local") {
            page = "explore"
        }
    }
</script>

<div class="flex">
    <aside class="sticky top-2 flex h-[41.2rem] w-20 flex-col items-center bg-custom2 rounded-lg ml-2 my-2">
        <div class="flex h-[4.5rem] w-full items-center justify-center p-2">
            <p class="text-white font-mono text-xl">Index App</p>
        </div>
        <nav class="flex flex-1 flex-col self-center gap-y-4 pt-10">
            <button on:click={switchPage} class="bg-custom4 text-white group relative rounded-lg px-3 py-2">
                {#if page === "explore"}
                    Local
                {:else if page === "local"}
                    Explore
                {/if}
            </button>
        </nav>
    </aside>

    <div class="overflow-y-scroll no-scrollbar">
        {#if page === "explore"}
            <ModListWindow/>
        {:else if page === "local"}
            <LocalMods />
        {/if}
    </div>
</div>