<script lang="ts">
  import ModListWindow from "./components/ModListWindow.svelte";
  import {onMount} from "svelte";
  import {BaseDirectory, createDir, exists} from "@tauri-apps/api/fs";
  import Sidebar from "./includes/Sidebar.svelte";

  onMount(async () => {
    if (!await (exists('.weave', {dir: BaseDirectory.Home}))) {
      await createDir(".weave/mods", {dir: BaseDirectory.Home})
    }
  })
</script>

<main>
  <div>
    <Sidebar />
  </div>
</main>
