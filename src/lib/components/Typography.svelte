<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
    component?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span";
    size?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl";
    color?: "default" | "subtle" | "error";
    classes?: string;
  }

  let { children, component = "p", color = "default", size }: Props = $props();

  const sizes = {
    xs: "text-xs",
    sm: "text-sm",
    md: "text-base",
    lg: "text-lg",
    xl: "text-xl",
    "2xl": "text-2xl",
  };

  const defaultSizes = {
    h1: "text-4xl",
    h2: "text-3xl",
    h3: "text-2xl",
    h4: "text-xl",
    h5: "text-lg",
    h6: "text-base",
    p: "text-base",
    span: "text-base",
    small: "text-sm",
  };

  const colors = {
    default: "text-default-text",
    subtle: "text-text-subtle",
    error: "text-error-text",
  };

  const sizeClass = $derived(size ? sizes[size] : defaultSizes[component]);

  const colorClass = $derived(colors[color]);
</script>

{#if component === "h1"}
  <h1 class="font-bold {colorClass} {sizeClass}">
    {@render children()}
  </h1>
{:else if component === "h2"}
  <h2 class="font-bold {colorClass} {sizeClass}">
    {@render children()}
  </h2>
{:else if component === "h3"}
  <h3 class="font-bold {colorClass} {sizeClass}">
    {@render children()}
  </h3>
{:else if component === "h4"}
  <h4 class="font-bold {colorClass} {sizeClass}">{@render children()}</h4>
{:else if component === "h5"}
  <h5 class="font-bold {colorClass} {sizeClass}">{@render children()}</h5>
{:else if component === "h6"}
  <h6 class="font-bold {colorClass} {sizeClass}">
    {@render children()}
  </h6>
{:else if component === "p"}
  <p class="text-base {colorClass} {sizeClass}">{@render children()}</p>
{:else if component === "span"}
  <span class="text-base {colorClass} {sizeClass}">{@render children()}</span>
{/if}
