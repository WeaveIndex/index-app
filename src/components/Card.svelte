<script lang="ts">
    import {Client, getClient, ResponseType} from '@tauri-apps/api/http';
    import {writeBinaryFile, createDir, exists, BaseDirectory} from '@tauri-apps/api/fs'
    import {onMount} from "svelte";

    export let title, desc, author, modurl;
    let httpClient: Client

    onMount(async () => {
        httpClient = await getClient()

        if (!await (exists('.weave', {dir: BaseDirectory.Home}))) {
            await createDir(".weave/mods", {dir: BaseDirectory.Home})
        }
    })

    async function downloadMod(url: String, modName: String) {
        const response = await httpClient.get(url, {
            responseType: ResponseType.Binary
        })

        let modDir = ".weave/mods/";

        await writeBinaryFile(
            modDir + modName + ".jar",
            response.data,
            {dir: BaseDirectory.Home}
        )

        await alert("Download/Update successfully completed")
    }
</script>

<div>
    <div class="block max-w-sm h-auto p-6 border border-gray-200 rounded-lg shadow bg-gray-800 border-gray-700">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-white">{title}</h5>
        <h6 class="mb-2 text-sm tracking-tight text-white">By {author}</h6>
        <p class="font-normal text-gray-700 dark:text-gray-400">{desc}</p>

        <div class="mt-4 flex justify-end">
            <button
              class="bg-gray-700 hover:bg-gray-900 p-2 rounded-lg transition duration-150 ease-in-out"
              on:click={async () => await downloadMod(modurl, title)}>Download
            </button>
        </div>
    </div>
</div>