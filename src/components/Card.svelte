<script lang="ts">
    import {Client, getClient, ResponseType} from '@tauri-apps/api/http';
    import {writeBinaryFile, BaseDirectory, exists, createDir} from '@tauri-apps/api/fs'
    import type {BinaryFileContents} from '@tauri-apps/api/fs'
    import {onMount} from "svelte";

    export let title: string, desc: string, author: string, modurl: string;
    let httpClient: Client

    onMount(async () => {
        httpClient = await getClient()

        if (!await (exists('.weave', {dir: BaseDirectory.Home}))) {
            await createDir(".weave/mods", {dir: BaseDirectory.Home})
        }
    })

    async function downloadMod(url: String, modName: String) {
        const response = await httpClient.get(`${url}`, {
            responseType: ResponseType.Binary
        })

        let modDir = ".weave/mods/";

        await writeBinaryFile(
            modDir + modName + ".jar",
            response.data as BinaryFileContents,
            {dir: BaseDirectory.Home}
        )

        await alert("Download/Update successfully completed")
    }
</script>

<div>
    <div class="block relative max-w-sm h-56 p-6 rounded-lg bg-custom2">
        <h5 class="mb-2 text-xl font-mono font-bold tracking-tight text-white">{title}</h5>
        <h6 class="mb-2 text-xs tracking-tight text-white">By {author}</h6>
        <p class="font-normal text-gray-700 dark:text-gray-400">{desc}</p>

        <div class="flex absolute bottom-4 items-center justify-end pt-2.5">
            <button
                    class="font-semibold font-mono rounded-md bg-custom3 hover:bg-custom1 transition duration-300 ease-in-out text-white p-2 px-4"
                    on:click={async () => await downloadMod(`${modurl}`, title)}>Download
            </button>
        </div>
    </div>
</div>
