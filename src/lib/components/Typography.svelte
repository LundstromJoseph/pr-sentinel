<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    variant?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span" | "small";
    color?: "default" | "error";
    classes?: string;
  }

  let {
    children,
    variant = "p",
    color = "default",
    classes = "",
  }: Props = $props();

  let colorClass = $derived(
    color === "error" ? "text-red-500" : "text-default-colors"
  );
</script>

{#if variant === "h1"}
  <h1 class="text-4xl font-bold {colorClass} {classes}">
    {@render children()}
  </h1>
{:else if variant === "h2"}
  <h2 class="text-3xl font-bold {colorClass} {classes}">
    {@render children()}
  </h2>
{:else if variant === "h3"}
  <h3 class="text-2xl font-bold {colorClass} {classes}">
    {@render children()}
  </h3>
{:else if variant === "h4"}
  <h4 class="text-xl font-bold {colorClass} {classes}">{@render children()}</h4>
{:else if variant === "h5"}
  <h5 class="text-lg font-bold {colorClass} {classes}">{@render children()}</h5>
{:else if variant === "h6"}
  <h6 class="text-base font-bold {colorClass} {classes}">
    {@render children()}
  </h6>
{:else if variant === "p"}
  <p class="text-base {colorClass} {classes}">{@render children()}</p>
{:else if variant === "span"}
  <span class="text-base {colorClass} {classes}">{@render children()}</span>
{:else if variant === "small"}
  <small class="text-sm {colorClass} {classes}">{@render children()}</small>
{/if}
