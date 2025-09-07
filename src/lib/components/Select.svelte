<script lang="ts">
  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    options: Option[];
    value: string;
    placeholder?: string;
    textColor?: "default" | "error";
  }

  let {
    options,
    value = $bindable(),
    placeholder,
    textColor = "default",
  }: Props = $props();

  const colors = {
    default: "text-text-default",
    error: "text-text-error",
  };

  let textColorClass = $derived(colors[textColor]);
</script>

<div class="relative">
  <select
    bind:value
    class="appearance-none w-full border-b {textColorClass} border-border-default focus:border-button-primary p-2 outline-none bg-transparent pr-8"
  >
    {#if placeholder}
      <option value="" disabled selected>{placeholder}</option>
    {/if}
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
  <div
    class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2"
  >
    <svg class="h-4 w-4 fill-text-default" viewBox="0 0 20 20">
      <path
        d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
      />
    </svg>
  </div>
</div>
