<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import Button from "$lib/components/Button.svelte";
  import ListButton from "$lib/components/ListButton.svelte";
  import SubtleButton from "$lib/components/SubtleButton.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import Cogwheel from "$lib/icons/Cogwheel.svelte";
  import Plus from "$lib/icons/Plus.svelte";
  import Refresh from "$lib/icons/Refresh.svelte";
  import Tray from "$lib/icons/Tray.svelte";
  import type { GithubFilter, PullRequestsData } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import AddFilterScreen from "./AddFilterScreen.svelte";
  import PullRequestListScreen from "./PullRequestListScreen.svelte";
  import SettingsScreen from "./SettingsScreen.svelte";

  type Screen =
    | {
        name: "new" | "settings";
      }
    | {
        name: "filter";
        id: string;
      };

  interface Props {
    appState: AppState;
  }

  let { appState }: Props = $props();

  let refreshing = $state(false);

  let screen = $state<Screen | null>(null);

  function close() {
    screen = null;
  }

  function formatDate(seconds: number) {
    return new Date(seconds * 1000).toLocaleString("sv-SE", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  let filtersWithData = $derived.by(() => {
    return appState.config.filters.map((filter) => {
      return {
        filter,
        data: appState.data.pull_requests[filter.id] ?? {
          last_updated: 0,
          pull_requests: [],
        },
      };
    });
  });

  let selectedFilter = $derived.by(() => {
    return filtersWithData.find((item) => {
      if (screen?.name === "filter") {
        return item.filter.id === screen.id;
      }
      return false;
    });
  });

  function isSelected(filter: GithubFilter) {
    return (
      screen &&
      typeof screen === "object" &&
      screen.name === "filter" &&
      screen.id === filter.id
    );
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
    <div class="flex justify-end gap-2 p-2 border-b border-border-default">
      <SubtleButton
        selected={screen?.name === "settings"}
        onClick={() => (screen = { name: "settings" })}
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
      <div class="grow"></div>
      <SubtleButton
        selected={screen?.name === "new"}
        onClick={() => (screen = { name: "new" })}
      >
        <Plus />
      </SubtleButton>
    </div>
    <menu class="flex flex-col">
      {#each filtersWithData as item}
        <ListButton
          classes="border-b rounded-none border-border-default flex flex-col flex-start"
          onClick={() => (screen = { name: "filter", id: item.filter.id })}
        >
          <div class="flex justify-between w-full">
            <Typography color={isSelected(item.filter) ? "default" : "subtle"}>
              {item.filter.name}
            </Typography>
            <Typography color={isSelected(item.filter) ? "default" : "subtle"}>
              {item.data.pull_requests.length}
            </Typography>
          </div>
          <Typography size="sm" color="subtle">
            {formatDate(item.data.last_updated)}
          </Typography>
        </ListButton>
      {/each}
    </menu>
  </aside>
  <section class="flex flex-col gap-2 p-2">
    {#if screen?.name === "new"}
      <AddFilterScreen {close} />
    {:else if screen?.name === "settings"}
      <SettingsScreen {appState} />
    {:else if selectedFilter}
      <PullRequestListScreen
        data={selectedFilter.data}
        filter={selectedFilter.filter}
        {close}
      />
    {:else}
      <div class="flex flex-col gap-2 items-center justify-center h-full">
        <Typography component="h5">Pick a filter to the left</Typography>
        <Tray width={200} height={200} color="neutral" />
      </div>
    {/if}
  </section>
</div>
