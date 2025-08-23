<script lang="ts">
  import Typography from "$lib/components/Typography.svelte";
  import MainScreen from "$lib/screens/MainScreen.svelte";
  import Onboarding from "$lib/screens/Onboarding.svelte";
  import type { AppConfig, AppData } from "$lib/types.js";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let { data } = $props();

  onMount(() => {
    const listener = listen<{ config: AppConfig }>(
      "app-config-updated",
      (event) => {
        data.state.then((state) => {
          state.config = event.payload.config;
        });
      }
    );

    const listener2 = listen<{ data: AppData }>("app-data-updated", (event) => {
      data.state.then((state) => {
        state.data = event.payload.data;
      });
    });

    return () => {
      listener.then((unlisten) => {
        unlisten();
      });
      listener2.then((unlisten) => {
        unlisten();
      });
    };
  });
</script>

<main class="h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
  {#await data.state}
    <Typography variant="h1">Loading...</Typography>
  {:then appState}
    {#if appState.config.github_token}
      <MainScreen {appState} />
    {:else}
      <Onboarding {appState} />
    {/if}
  {/await}
</main>
