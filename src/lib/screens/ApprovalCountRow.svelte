<script lang="ts">
  import Select from "$lib/components/Select.svelte";
  import Typography from "$lib/components/Typography.svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    repoName: string;
    neededApprovals: number;
  }

  let { repoName, neededApprovals }: Props = $props();

  let state = {
    get neededApprovals() {
      return neededApprovals.toString();
    },
    set neededApprovals(value: string) {
      invoke("save_repo_config", {
        repoName,
        neededApprovals: parseInt(value),
      });
    },
  };
</script>

<Typography>{repoName}</Typography>
<Select
  options={[
    { value: "1", label: "1" },
    { value: "2", label: "2" },
    { value: "3", label: "3" },
    { value: "4", label: "4" },
    { value: "5", label: "5" },
  ]}
  bind:value={state.neededApprovals}
/>
