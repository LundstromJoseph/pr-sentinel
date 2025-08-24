<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<div
  class="px-4 py-2 rounded-full text-sm font-medium bg-button-subtle text-default-text w-fit"
>
  {@render children()}
</div>
