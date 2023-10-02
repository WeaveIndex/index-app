<script lang="ts">
    import Card from "./Card.svelte";
    import {onMount} from "svelte";
    import LocalMods from "./LocalMods.svelte";
    import {Client, getClient, ResponseType, fetch} from "@tauri-apps/api/http";
    import {BaseDirectory, writeBinaryFile} from "@tauri-apps/api/fs";
    import { Command } from '@tauri-apps/api/shell'
    import { invoke } from '@tauri-apps/api/tauri'

    let httpClient: Client


    type ModList = {
        name: string
        dl: string
        description: string
    }

    const endpoint: string = "https://cat-girl.tech/api/all";
    let mods: ModList[] = [];

    onMount(async function () {
        const response = await fetch(endpoint);
        httpClient = await getClient()

        //@ts-ignore i did this cuz ts is weird
        mods = response.data
    });

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
    <div class="flex flex-row gap-4">
        <div class="grid grid-cols-3 gap-3">
            {#each mods as data}
                <Card title={data.name} author={"i forgot to add author to api lmao"} modurl={data.dl} desc={data.description}/>
            {/each}
        </div>
    </div>
</div>