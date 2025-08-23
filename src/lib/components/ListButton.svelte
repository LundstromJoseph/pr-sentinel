<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    onClick?: () => void;
    enabled?: boolean;
    classes?: string;
  }

  let { children, onClick, enabled = true, classes = "" }: Props = $props();
</script>

<button
  class="cursor-pointer text-default-colors flex gap-2 p-4 border-b border-default-colors disabled:opacity-50 disabled:cursor-not-allowed {classes}"
  type={onClick ? "button" : "submit"}
  disabled={!enabled}
  onclick={onClick}
>
  {@render children()}
</button>
