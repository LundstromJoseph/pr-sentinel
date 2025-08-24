<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    onClick?: () => void;
    enabled?: boolean;
    classes?: string;
    selected?: boolean;
  }

  let {
    children,
    onClick,
    enabled = true,
    classes = "",
    selected = false,
  }: Props = $props();
</script>

<button
  data-selected={selected}
  class="cursor-pointer bg-button-subtle border border-button-subtle data-[selected=true]:border-white text-default-text flex justify-center gap-2 p-2 rounded-md disabled:opacity-50 disabled:cursor-not-allowed {classes}"
  type={onClick ? "button" : "submit"}
  disabled={!enabled}
  onclick={onClick}
>
  {@render children()}
</button>
