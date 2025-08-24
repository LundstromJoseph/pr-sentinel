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
  class="cursor-pointer bg-button-primary text-text-default flex justify-center gap-2 py-2 px-4 rounded-md disabled:opacity-50 disabled:cursor-not-allowed {classes}"
  type={onClick ? "button" : "submit"}
  disabled={!enabled}
  onclick={onClick}
>
  {@render children()}
</button>
