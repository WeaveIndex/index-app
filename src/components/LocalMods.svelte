<script lang="ts">
    //ripped from weave manager ty max
    import {watchImmediate} from "tauri-plugin-fs-watch-api"
    import {BaseDirectory, readDir, removeFile, renameFile, writeBinaryFile} from "@tauri-apps/api/fs";
    import {homeDir} from "@tauri-apps/api/path";
    import {onMount} from "svelte"
    import {Client, getClient, ResponseType} from "@tauri-apps/api/http";
    import type {BinaryFileContents} from '@tauri-apps/api/fs'

    type Mod = {
        name: string
        path: string
    }

    let modList: Mod[] = []
    let httpClient: Client

    async function updateModList() {
        const entries = await readDir('.weave/mods', {dir: BaseDirectory.Home})
        modList = await Promise.all(entries
            .filter(e => e.name.includes('.jar') && e.children == null)
            .map(async (e) => <Mod>{
                path: e.path,
                name: e.name
            })
        )

    }

    async function disableMod(modPath: string) {
        await renameFile(modPath, modPath + '.disabled')
    }

    async function enableMod(modPath: string) {
        await renameFile(modPath, modPath.replace('.disabled', ''))
    }

    async function deleteMod(modPath: string) {
        // this function took me 10 min
        await removeFile(modPath)
    }

    onMount(async () => {
        await updateModList()
        httpClient = await getClient()

        await watchImmediate(
            `${await homeDir()}/.weave/mods`,
            async () => await updateModList()
        );
    });

    const testWeave = async () => {
        const response = await httpClient.get("https://gitlab.com/candicey-weave/weave-diagnose/uploads/a7dce0a0ea856a403184dbb909e19354/WeaveDiagnose.jar", {
            responseType: ResponseType.Binary
        })

        let modDir = ".weave/";

        await writeBinaryFile(
            modDir + "WeaveDiagnose.jar",
            response.data as BinaryFileContents,
            {dir: BaseDirectory.Home}
        )
    }
</script>

<div class="h-[41.2rem] w-[64rem] mt-2 ml-2 rounded-lg bg-custom2">
    <div class="flex items-center justify-center">
        <p class="text-white mt-4 text-2xl font-semibold font-mono">Installed mods</p>
    </div>

    <div class="bg-custom1 rounded-lg m-2 grid grid-cols-3 gap-3 px-4 py-2">
        {#each modList as data (data.path)}
            <div>
                <div class="block relative max-w-sm h-32 p-6 rounded-lg bg-custom2">
                    <h5 class="mb-2 text-xl font-mono font-bold tracking-tight text-white">{data.name}</h5>

                    <div class="flex absolute bottom-4 items-center justify-end pt-2.5">
                        <button on:click={async () => await deleteMod(data.path)} class="mr-2 font-semibold rounded-md bg-custom3 hover:bg-custom1 transition duration-300 ease-in-out text-white p-2">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                                <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                                      stroke-width="2"
                                      d="M14 11v6m-4-6v6M6 7v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7M4 7h16M7 7l2-4h6l2 4"/>
                            </svg>
                        </button>

                        {#if data.path.endsWith('.disabled')}
                            <button
                                    class="font-semibold font-mono rounded-md bg-custom3 hover:bg-custom1 transition duration-300 ease-in-out text-white p-2 px-4"
                                    on:click={async() => await enableMod(data.path)}>
                                Enable
                            </button>
                        {:else}
                            <button
                                    class="font-semibold font-mono rounded-md bg-custom3 hover:bg-custom1 transition duration-300 ease-in-out text-white p-2 px-4"
                                    on:click={async() => await disableMod(data.path)}>
                                Disable
                            </button>
                        {/if}
                    </div>
                </div>
            </div>
        {/each}
    </div>
</div>

<!--
<div class="mt-2 h-28 rounded-lg bg-gray-800 border-gray-700 border shadow">
    <div class="flex mt-8 justify-center items-center">
        <button class="text-white bg-gray-700 hover:bg-gray-900 p-2 rounded-lg transition duration-150 ease-in-out">
            Test Weave
        </button>
    </div>
    <p class="text-white mt-2 ml-2">Powered by weave diagnose©</p>
</div>
-->