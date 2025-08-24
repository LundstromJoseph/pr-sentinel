<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import Checkbox from "$lib/components/Checkbox.svelte";
  import GithubTokenInput from "$lib/components/GithubTokenInput.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import { enable, isEnabled, disable } from "@tauri-apps/plugin-autostart";

  interface Props {
    appState: AppState;
  }

  let isRunOnStartupEnabled = $state(false);

  $effect(() => {
    isEnabled().then((value) => {
      isRunOnStartupEnabled = value;
    });
  });

  let { appState }: Props = $props();

  let runOnStartup = {
    get value() {
      return isRunOnStartupEnabled;
    },
    set value(value: boolean) {
      if (value) {
        enable().then(() => {
          isRunOnStartupEnabled = true;
        });
      } else {
        disable().then(() => {
          isRunOnStartupEnabled = false;
        });
      }
    },
  };
</script>

<section class="p-4 flex flex-col gap-6">
  <Typography component="h3">Settings</Typography>
  <GithubTokenInput {appState} />
  <section class="p-2">
    <Checkbox bind:value={runOnStartup.value} label="Run on startup" />
  </section>
</section>
