<script lang="ts">
    import Card from "./Card.svelte";
    import {onMount} from "svelte";
    import {Client, getClient, ResponseType, fetch} from "@tauri-apps/api/http";
    import {BaseDirectory, writeBinaryFile} from "@tauri-apps/api/fs";
    import {invoke} from '@tauri-apps/api/tauri'
    import NoResult from "../includes/NoResult.svelte";

    let httpClient: Client

    type ModList = {
        name: string
        dl: string
        description: string
        time: number
    }

    let search = "";
    let filteredSearch = [];

    const endpoint: string = "https://cat-girl.tech/api/all";
    let mods: ModList[] = [];

    onMount(async function () {
        const response = await fetch(endpoint);
        httpClient = await getClient()

        //@ts-ignore i did this cuz ts is weird
        mods = response.data
    });

    const searchMod = () => {
        return filteredSearch = mods.filter(book => {
            let bookTitle = book.name.toLowerCase()
            return bookTitle.includes(search.toLowerCase())
        });
    }

    const testWeave = async () => {
        const response = await httpClient.get("https://gitlab.com/candicey-weave/weave-diagnose/uploads/a7dce0a0ea856a403184dbb909e19354/WeaveDiagnose.jar", {
            responseType: ResponseType.Binary
        })

        let modDir = ".weave/";

        await writeBinaryFile(
            modDir + "WeaveDiagnose.jar",
            //@ts-ignore
            response.data,
            {dir: BaseDirectory.Home}
        )

        await invoke("launch_diagnoser")
    }
</script>

<div class="m-4">
    <div class="mb-4">
        <div class='max-w'>
            <div class="relative flex items-center w-[63rem] h-12 rounded-lg focus-within:shadow-lg bg-custom2 overflow-hidden">
                <div class="grid place-items-center h-full w-12 text-white">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                         stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                    </svg>
                </div>

                <input
                        bind:value={search}
                        on:input={searchMod}
                        class="peer bg-custom2 h-full w-full outline-none text-sm text-white pr-2"
                        type="text"
                        id="search"
                        placeholder="Search here"/>
            </div>
        </div>
    </div>
    <div class="flex flex-row gap-4">
        <div class="grid grid-cols-3 gap-3">
            {#if search && filteredSearch.length === 0}
                <NoResult/>
            {:else if filteredSearch.length > 0}
                {#each filteredSearch as data}
                    <Card title={data.name} author={"i forgot to add author to api lmao"} modurl={data.dl}
                          desc={data.description}/>
                {/each}
            {:else}
                {#each mods as data}
                    <Card title={data.name} author={"i forgot to add author to api lmao"} modurl={data.dl}
                          desc={data.description}/>
                {/each}
            {/if}
        </div>
    </div>
</div>