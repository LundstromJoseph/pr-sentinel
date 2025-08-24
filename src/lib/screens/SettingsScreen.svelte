<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import GithubTokenInput from "$lib/components/GithubTokenInput.svelte";
  import Typography from "$lib/components/Typography.svelte";

  interface Props {
    appState: AppState;
  }

  let { appState }: Props = $props();
</script>

<section class="p-4 flex flex-col gap-6">
  <Typography variant="h3">Settings</Typography>
  <GithubTokenInput {appState} />
</section>
