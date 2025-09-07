<script lang="ts">
  import Typography from "$lib/components/Typography.svelte";
  import MainScreen from "$lib/screens/MainScreen.svelte";
  import Onboarding from "$lib/screens/Onboarding.svelte";
  import type { AppConfig, AppData } from "$lib/types.js";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let { data } = $props();

  onMount(() => {
    const configListener = listen<{ config: AppConfig }>(
      "app-config-updated",
      (event) => {
        data.state.then((state) => {
          state.config = event.payload.config;
        });
      }
    );

    const dataListener = listen<{ data: AppData }>(
      "app-data-updated",
      (event) => {
        data.state.then((state) => {
          state.data = event.payload.data;
        });
      }
    );

    return () => {
      configListener.then((unlisten) => {
        unlisten();
      });
      dataListener.then((unlisten) => {
        unlisten();
      });
    };
  });
</script>

<main class="h-screen bg-bg-default flex flex-col">
  {#await data.state}
    <Typography component="h1">Loading...</Typography>
  {:then appState}
    {#if appState.config.github_token}
      <MainScreen {appState} />
    {:else}
      <Onboarding {appState} />
    {/if}
  {/await}
</main>
