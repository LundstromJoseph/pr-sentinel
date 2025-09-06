<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import ListButton from "$lib/components/ListButton.svelte";
  import SubtleButton from "$lib/components/SubtleButton.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import Cogwheel from "$lib/icons/Cogwheel.svelte";
  import Refresh from "$lib/icons/Refresh.svelte";
  import Tray from "$lib/icons/Tray.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import PullRequestListScreen from "./PullRequestListScreen.svelte";
  import SettingsScreen from "./SettingsScreen.svelte";
  import { categories } from "$lib/domain/categories";
  import type { PullRequestItem } from "$lib/types";

  type Category = (typeof categories)[number];

  type Screen =
    | { key: "settings" }
    | (Category & { pullRequests: PullRequestItem[] });

  interface Props {
    appState: AppState;
  }

  let { appState }: Props = $props();

  let refreshing = $state(false);

  let screen = $state<Screen | null>(null);

  let categoriesWithPrs = $derived(
    categories.map((category) => ({
      ...category,
      pullRequests: appState.data.pull_requests.pull_requests.filter((pr) =>
        category.prCategories.includes(pr.category)
      ),
    }))
  );

  const levelToPadding: Record<number, string> = {
    0: "pl-4",
    1: "pl-6",
  };

  function formatDate(seconds: number) {
    return new Date(seconds * 1000).toLocaleString("sv-SE", {
      year: "numeric",
      month: "long",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  async function refresh() {
    refreshing = true;
    await invoke("refresh");
    refreshing = false;
  }
</script>

<div
  class="grid grid-cols-[minmax(12rem,1fr)_6fr] grid-rows-1 w-full h-full flex-1"
>
  <aside
    class="flex flex-col overflow-y-auto min-h-0 border-r border-border-default"
  >
    <menu class="flex flex-col">
      {#each categoriesWithPrs as category}
        <ListButton
          classes="border-b rounded-none border-border-default flex flex-col flex-start {levelToPadding[
            category.level
          ]}"
          onClick={() => (screen = category)}
        >
          <div class="flex justify-between w-full">
            <Typography
              size={"sm"}
              color={category.key === screen?.key ? "default" : "subtle"}
            >
              {category.name}
            </Typography>
            <Typography
              size={"sm"}
              color={category.key === screen?.key ? "default" : "subtle"}
            >
              {category.pullRequests.length}
            </Typography>
          </div>
        </ListButton>
      {/each}
    </menu>
    <div class="flex-grow"></div>
    <div class="flex justify-start p-2 gap-2 border-t border-border-default">
      <SubtleButton
        selected={screen?.key === "settings"}
        onClick={() => (screen = { key: "settings" })}
      >
        <Cogwheel />
      </SubtleButton>

      <SubtleButton onClick={refresh}>
        {#if refreshing}
          <Refresh color="subtle" />
        {:else}
          <Refresh />
        {/if}
      </SubtleButton>
    </div>
  </aside>
  <section class="flex flex-col p-2 overflow-y-auto">
    {#if screen?.key === "settings"}
      <SettingsScreen {appState} />
    {:else if screen && "name" in screen}
      <PullRequestListScreen data={screen.pullRequests} />
    {:else}
      <div class="flex flex-col gap-2 items-center justify-center h-full">
        <Typography component="h5">Pick a filter to the left</Typography>
        <Tray width={200} height={200} color="neutral" />
      </div>
    {/if}

    <div class="flex-grow"></div>

    <footer class="flex flex-col pt-1 items-end">
      <Typography size="sm" color="subtle">PRs last updated:</Typography>
      <Typography size="sm" color="subtle"
        >{formatDate(appState.data.pull_requests.last_updated)}</Typography
      >
    </footer>
  </section>
</div>
