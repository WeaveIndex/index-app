<script lang="ts">
    import Card from "./Card.svelte";

    import { onMount } from "svelte";
    const endpoint = "https://raw.githubusercontent.com/WeaveIndexRepoBot/ModRepo/main/data/mods.json";
    let posts = [];

    onMount(async function () {
        const response = await fetch(endpoint);
        const data = await response.json();

        posts = data

        console.log(data);
    });
</script>

<div class="w-full h-full">
    <div class="m-4 h-full pb-8 text-white overflow-y-auto">
        <div class="w-full flex flex-col">
                <div class="grid grid-cols-2 gap-2">
                    {#each posts as data}
                        <Card title={data.name} author={data.author} modurl={data.murl} desc={data.shortdesc}/>
                    {/each}
                </div>
        </div>
    </div>
</div>