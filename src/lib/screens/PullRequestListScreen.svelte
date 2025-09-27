<script lang="ts">
  import PullRequestRow from "$lib/components/PullRequestRow.svelte";
  import type { PullRequestItem } from "$lib/types";

  interface Props {
    data: PullRequestItem[];
  }

  let { data }: Props = $props();
  let showAssignedOnly = $state(false);

  let filteredData = $derived(
    showAssignedOnly ? data.filter((pr) => pr.is_assigned) : data
  );
</script>

<div class="flex flex-col w-full">
  <div
    class="flex items-center gap-4 p-4 border-b border-gray-200 dark:border-gray-700"
  >
    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={showAssignedOnly}
        class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700"
      />
      <span class="text-sm font-medium text-gray-700 dark:text-gray-200">
        Show assigned only
      </span>
    </label>
  </div>

  <div class="flex flex-col">
    {#each filteredData as pullRequest}
      <PullRequestRow {pullRequest} />
    {/each}
  </div>
</div>
