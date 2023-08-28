<script lang="ts">
    import Card from "./Card.svelte";
    import {onMount} from "svelte";
    import LocalMods from "./LocalMods.svelte";

    type ModList = {
        name: string
        author: string
        murl: string
        shortdesc: string
    }

    const endpoint: string = "https://raw.githubusercontent.com/WeaveIndexRepoBot/ModRepo/main/data/mods.json";
    let mods: ModList[] = [];

    onMount(async function () {
        const response = await fetch(endpoint);
        const data = await response.json();

        mods = data

        console.log(data);
    });
</script>

<div class="m-4">
    <div class="flex flex-row gap-4">
        <div class="grid grid-cols-2 gap-2">
            {#each mods as data}
                <Card title={data.name} author={data.author} modurl={data.murl} desc={data.shortdesc}/>
            {/each}
        </div>

        <div>
            <LocalMods />
        </div>
    </div>
</div>