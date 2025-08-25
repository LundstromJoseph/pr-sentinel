<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import GithubTokenInput from "$lib/components/GithubTokenInput.svelte";
  import Typography from "$lib/components/Typography.svelte";

  interface Props {
    appState: AppState;
  }

  let { appState }: Props = $props();
</script>

<section class="flex w-full flex-col gap-6 p-2 justify-center items-center">
  <Typography component="h3">No GitHub token found, please add one</Typography>

  <div class="w-full">
    <GithubTokenInput {appState} />
  </div>
</section>
