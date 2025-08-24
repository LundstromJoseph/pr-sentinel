<script lang="ts">
  import Pill from "$lib/components/Pill.svelte";
  import PullRequestRow from "$lib/components/PullRequestRow.svelte";
  import SubtleButton from "$lib/components/SubtleButton.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import Cross from "$lib/icons/Cross.svelte";
  import Link from "$lib/icons/Link.svelte";
  import type { GithubFilter, PullRequestsData } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import EditFilterScreen from "./EditFilterScreen.svelte";
  import Pen from "$lib/icons/Pen.svelte";
  import UpArrow from "$lib/icons/UpArrow.svelte";
  import DownArrow from "$lib/icons/DownArrow.svelte";
  import Bell from "$lib/icons/Bell.svelte";

  interface Props {
    data: PullRequestsData;
    filter: GithubFilter;
    close: () => void;
  }

  let editingFilter = $state(false);

  $effect(() => {
    if (filter) {
      editingFilter = false;
    }
  });

  let { data, filter, close }: Props = $props();

  async function deleteFilter() {
    await invoke("remove_filter", { id: filter.id }).catch((error) => {
      console.error(error);
    });
    close();
  }

  async function reorderFilter(direction: "up" | "down") {
    await invoke("reorder_filter", {
      filterId: filter.id,
      direction,
    }).catch((error) => {
      console.error(error);
    });
  }

  function openGithub(query: string) {
    const url = `https://github.com/pulls?q=${encodeURIComponent(query)}`;
    openUrl(url);
  }
</script>

<div class="flex flex-col gap-2 overflow-y-auto">
  {#if editingFilter}
    <EditFilterScreen {filter} onclose={() => (editingFilter = false)} />
  {:else}
    <header class="border-b border-border-strong flex flex-col gap-4 pb-4 px-2">
      <div class="flex gap-2 items-center justify-between">
        <div class="flex gap-2 items-center">
          <Typography component="h5">{filter.name}</Typography>
          {#if filter.notify}
            <Bell />
          {/if}
          <SubtleButton onClick={() => openGithub(filter.query)}>
            <Link />
          </SubtleButton>
        </div>
        <div class="flex gap-2 items-center">
          <SubtleButton onClick={() => reorderFilter("up")}>
            <UpArrow />
          </SubtleButton>
          <SubtleButton onClick={() => reorderFilter("down")}>
            <DownArrow />
          </SubtleButton>
          <SubtleButton onClick={() => (editingFilter = true)}>
            <Pen />
          </SubtleButton>
          <SubtleButton onClick={deleteFilter}>
            <Cross color="error" />
          </SubtleButton>
        </div>
      </div>

      <Pill>{filter.query}</Pill>
    </header>
    {#each data.pull_requests as pullRequest}
      <PullRequestRow {pullRequest} />
    {/each}
  {/if}
</div>
