<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<span class="px-2 py-1 rounded-full text-sm font-medium bg-gray-800 text-white">
  {@render children()}
</span>
