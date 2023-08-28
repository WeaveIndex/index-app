<script lang="ts">
    //ripped from weave manager ty max
    import {watchImmediate} from "tauri-plugin-fs-watch-api"
    import {BaseDirectory, readDir, renameFile} from "@tauri-apps/api/fs";
    import {homeDir} from "@tauri-apps/api/path";
    import {onMount} from "svelte"

    type Mod = {
        name: string
        path: string
    }

    let modList: Mod[] = []

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

    onMount(async () => {
        await updateModList()

        await watchImmediate(
            `${await homeDir()}/.weave/mods`,
            async () => await updateModList()
        );
    });
</script>

<div class="h-[45rem] w-96 rounded-lg bg-gray-800 border-gray-700 border shadow">
    <div class="flex items-center justify-center">
        <p class="text-white text-2xl font-bold">Installed mods</p>
    </div>

    <div class="flex flex-col px-4 py-2">
        {#each modList as data (data.path)}
            <div>
                <p class="text-white">{data.name}</p>
            </div>

            <div class="text-right">
                {#if data.path.endsWith('.disabled')}
                    <button
                            class="text-white bg-gray-700 hover:bg-gray-900 p-2 rounded-lg transition duration-150 ease-in-out"
                            on:click={async() => await enableMod(data.path)}>
                        Enable
                    </button>
                {:else}
                    <button
                            class="text-white bg-gray-700 hover:bg-gray-900 p-2 rounded-lg transition duration-150 ease-in-out"
                            on:click={async() => await disableMod(data.path)}>
                        Disable
                    </button>
                {/if}
            </div>
        {/each}
    </div>
</div>