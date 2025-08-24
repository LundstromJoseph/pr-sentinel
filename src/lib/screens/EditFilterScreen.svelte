<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import Checkbox from "$lib/components/Checkbox.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import type { GithubFilter } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    filter: GithubFilter;
    onclose: () => void;
  }
  let { filter, onclose }: Props = $props();

  let filterName = $state(filter.name);
  let filterQuery = $state(filter.query);
  let notify = $state(filter.notify);
  let error = $state<string | null>(null);

  async function handleSubmit(event: Event) {
    event.preventDefault();
    error = null;
    invoke("update_filter", {
      id: filter.id,
      filter: {
        name: filterName,
        query: filterQuery,
        notify: notify,
      },
    })
      .then(() => {
        onclose();
      })
      .catch((err) => {
        error = err;
      });
  }
</script>

<section class="p-4 flex flex-col gap-6">
  <Typography variant="h3">Edit filter</Typography>
  <form onsubmit={handleSubmit} class="flex flex-col gap-4">
    <div class="flex flex-col">
      <Typography variant="p">Filter name</Typography>
      <TextInput bind:value={filterName} />
    </div>
    <div class="flex flex-col">
      <Typography variant="p">Filter query</Typography>
      <TextInput bind:value={filterQuery} />
    </div>
    <div class="flex gap-2 items-center">
      <Checkbox bind:value={notify} label="Notify" />
    </div>
    <Button>Save</Button>
  </form>
  {#if error}
    <Typography variant="p" color="error">{error}</Typography>
  {/if}
</section>
