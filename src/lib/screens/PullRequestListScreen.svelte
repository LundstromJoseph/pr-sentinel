<script lang="ts">
  import PullRequestRow from "$lib/components/PullRequestRow.svelte";
  import type { PullRequestItem } from "$lib/types";

  interface Props {
    data: PullRequestItem[];
  }

  let { data }: Props = $props();
</script>

{#each data as pullRequest}
  <PullRequestRow {pullRequest} />
{/each}
