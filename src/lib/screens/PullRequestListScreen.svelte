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

  interface Props {
    data: PullRequestsData;
    filter: GithubFilter;
    close: () => void;
  }

  let { data, filter, close }: Props = $props();

  async function deleteFilter() {
    await invoke("remove_filter", { id: filter.id }).catch((error) => {
      console.error(error);
    });
    close();
  }

  function openGithub(query: string) {
    const url = `https://github.com/pulls?q=${encodeURIComponent(query)}`;
    openUrl(url);
  }
</script>

<div class="flex flex-col gap-2 overflow-y-auto">
  <header class="border-b border-gray-300 flex flex-col gap-4 pb-4 px-2">
    <div class="flex gap-2 items-center justify-between">
      <div class="flex gap-2 items-center">
        <Typography variant="h5">{filter.name}</Typography>
        <SubtleButton onClick={() => openGithub(filter.query)}>
          <Link />
        </SubtleButton>
      </div>
      <SubtleButton onClick={deleteFilter}>
        <Cross color="red" />
      </SubtleButton>
    </div>

    <Pill>{filter.query}</Pill>
  </header>
  {#each data.pull_requests as pullRequest}
    <PullRequestRow {pullRequest} />
  {/each}
</div>
