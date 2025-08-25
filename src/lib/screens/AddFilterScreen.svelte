<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import Checkbox from "$lib/components/Checkbox.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    close: () => void;
  }

  let { close }: Props = $props();

  let filterName = $state("");
  let filterQuery = $state("");
  let notify = $state(false);
  let error = $state<string | null>(null);

  async function handleSubmit(event: Event) {
    event.preventDefault();
    error = null;
    await invoke("add_filter", {
      filter: {
        name: filterName,
        query: filterQuery,
        notify: notify,
      },
    })
      .catch((err) => {
        error = err;
      })
      .then(() => {
        close();
      });
  }
</script>

<section class="p-4 flex flex-col gap-6">
  <Typography component="h3">Add filter</Typography>
  <form onsubmit={handleSubmit} class="flex flex-col gap-4">
    <div class="flex flex-col">
      <Typography component="p">Filter name</Typography>
      <TextInput bind:value={filterName} />
    </div>
    <div class="flex flex-col">
      <Typography component="p">Filter query</Typography>
      <TextInput bind:value={filterQuery} />
    </div>
    <div class="flex gap-2 items-center">
      <Checkbox bind:value={notify} label="Notify" />
    </div>
    <Button>Add filter</Button>
  </form>
  {#if error}
    <Typography component="p" color="error">{error}</Typography>
  {/if}
</section>
