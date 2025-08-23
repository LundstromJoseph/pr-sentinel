<script lang="ts">
  import type { PullRequestItem } from "$lib/types";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import ListButton from "./ListButton.svelte";
  import Typography from "./Typography.svelte";

  interface Props {
    pullRequest: PullRequestItem;
  }

  let { pullRequest }: Props = $props();

  let updatedAt = $derived(
    new Date(pullRequest.updated_at).toLocaleString("sv-SE", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    })
  );

  function handleClick() {
    openUrl(pullRequest.pull_request.html_url);
  }

  let repositoryName = $derived(
    pullRequest.repository_url.split("repos/")[1] ?? "Unknown"
  );
</script>

<ListButton onClick={handleClick}>
  <div class="flex flex-col gap-2 justify-start items-start text-start">
    <div class="flex flex-row gap-2 items-center">
      <img
        src={pullRequest.user.avatar_url}
        alt={pullRequest.user.login}
        class="w-10 h-10 rounded-full"
      />
      <Typography variant="p">{pullRequest.title}</Typography>
    </div>
    <Typography variant="small">{repositoryName}</Typography>
    <Typography variant="small" classes="text-gray-500">
      {updatedAt}
    </Typography>
  </div>
</ListButton>
