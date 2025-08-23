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
  class="cursor-pointer bg-gray-800 border border-gray-800 data-[selected=true]:border-white text-white flex justify-center gap-2 p-2 rounded-md disabled:opacity-50 disabled:cursor-not-allowed {classes}"
  type={onClick ? "button" : "submit"}
  disabled={!enabled}
  onclick={onClick}
>
  {@render children()}
</button>
