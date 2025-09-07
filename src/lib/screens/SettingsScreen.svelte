<script lang="ts">
  import type { AppState } from "$lib/appstate.svelte";
  import Checkbox from "$lib/components/Checkbox.svelte";
  import GithubTokenInput from "$lib/components/GithubTokenInput.svelte";
  import ApprovalCountRow from "$lib/screens/ApprovalCountRow.svelte";
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

  let repoConfigs = $derived.by(() => {
    const newList = [...appState.config.repo_config];
    newList.sort((a, b) => a.repo_name.localeCompare(b.repo_name));
    return newList;
  });

  let runOnStartup = {
    get value() {
      return isRunOnStartupEnabled;
    },
    set value(value: boolean) {
      if (value) {
        enable()
          .then(() => {
            isRunOnStartupEnabled = true;
          })
          .catch((e) => {
            console.error(e);
          });
      } else {
        disable()
          .then(() => {
            isRunOnStartupEnabled = false;
          })
          .catch((e) => {
            console.error(e);
          });
      }
    },
  };
</script>

<section class="p-4 flex flex-col gap-6">
  <Typography component="h3">Settings</Typography>
  <GithubTokenInput {appState} />
  <section class="p-2 grid gap-2">
    <Typography component="h5">Run on startup</Typography>
    <div class="grid gap-2 p-2">
      <Checkbox bind:value={runOnStartup.value} label="Run on startup" />
    </div>
  </section>
  <section class="p-2 grid gap-2">
    <Typography component="h5">Approval counts</Typography>
    <div
      class="grid grid-cols-[fit-content(50%)_fit-content(50%)] p-2 gap-6 items-center"
    >
      {#each repoConfigs as repoConfig}
        <ApprovalCountRow
          repoName={repoConfig.repo_name}
          neededApprovals={repoConfig.needed_approvals}
        />
      {/each}
    </div>
  </section>
</section>
